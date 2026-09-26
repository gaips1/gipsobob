use super::types::*;
use crate::{modules::sbp::casino::mines::CASINO_MINES_SECRET_KEY, types::*};
use hmac::{Hmac, KeyInit, Mac};
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use rust_decimal::Decimal;
use sha2::Sha256;
use std::str::FromStr;
use subtle::ConstantTimeEq;

use poise::serenity_prelude::*;

type HmacSha256 = Hmac<Sha256>;

impl MinesGame {
    pub fn new(user_id: serenity::UserId, bet: Decimal, bombs: u8) -> Self {
        let seed = format!("{:08x}", rand::random::<u32>());
        Self {
            user_id,
            action: MinesAction::Tile(0),
            bet,
            bombs,
            seed,
            mask: 0,
        }
    }

    pub fn opened_count(&self) -> u32 {
        self.mask.count_ones()
    }

    fn is_opened(&self, index: u8) -> bool {
        (self.mask & (1 << index)) != 0
    }

    fn generate_sig(
        user_id: serenity::UserId,
        bet: &Decimal,
        bombs: u8,
        seed: &str,
        mask: u16,
    ) -> String {
        let mut mac = HmacSha256::new_from_slice(CASINO_MINES_SECRET_KEY.as_bytes()).unwrap();
        let payload = format!("{}:{}:{}:{}:{:x}", user_id, bet, bombs, seed, mask);
        mac.update(payload.as_bytes());
        let result = mac.finalize().into_bytes();
        hex::encode(&result[..8])
    }

    pub fn from_custom_id(custom_id: &str, user_id: serenity::UserId) -> Result<Self, MinesError> {
        let data = custom_id
            .strip_prefix("casino:mines:")
            .ok_or(MinesError::InvalidFormat)?;

        let parts: Vec<&str> = data.split(':').collect();
        if parts.len() != 6 {
            return Err(MinesError::InvalidFormat);
        }

        let action_str = parts[0];
        let bet_str = parts[1];
        let bombs_str = parts[2];
        let seed_str = parts[3];
        let mask_str = parts[4];
        let sig_str = parts[5];

        let bet = Decimal::from_str(bet_str).map_err(|e| MinesError::ParseError(e.to_string()))?;
        let bombs = bombs_str
            .parse::<u8>()
            .map_err(|e| MinesError::ParseError(e.to_string()))?;
        let mask =
            u16::from_str_radix(mask_str, 16).map_err(|e| MinesError::ParseError(e.to_string()))?;

        let expected_sig = Self::generate_sig(user_id, &bet, bombs, seed_str, mask);

        if expected_sig
            .as_bytes()
            .ct_eq(sig_str.as_bytes())
            .unwrap_u8()
            != 1
        {
            return Err(MinesError::InvalidSignature);
        }

        if bet <= Decimal::ZERO {
            return Err(MinesError::InvalidBet);
        }
        if bombs == 0 || bombs >= 16 {
            return Err(MinesError::InvalidBombCount);
        }

        let action = if action_str == "cashout" || action_str == "c" {
            MinesAction::Cashout
        } else {
            let tile_idx = action_str
                .parse::<u8>()
                .map_err(|e| MinesError::ParseError(e.to_string()))?;

            if tile_idx >= 16 {
                return Err(MinesError::InvalidTileIndex(tile_idx));
            }

            if (mask & (1 << tile_idx)) != 0 {
                return Err(MinesError::AlreadyOpened(tile_idx));
            }

            MinesAction::Tile(tile_idx)
        };

        Ok(Self {
            user_id,
            action,
            bet,
            bombs,
            seed: seed_str.to_string(),
            mask,
        })
    }

    pub fn to_custom_id(&self, action: &MinesAction) -> String {
        let action_str = match action {
            MinesAction::Tile(idx) => idx.to_string(),
            MinesAction::Cashout => "c".to_string(),
        };

        let sig = Self::generate_sig(self.user_id, &self.bet, self.bombs, &self.seed, self.mask);

        format!(
            "casino:mines:{}:{}:{}:{}:{:x}:{}",
            action_str, self.bet, self.bombs, self.seed, self.mask, sig
        )
    }

    fn multiplier_at_step(&self, step: u32) -> Decimal {
        if step == 0 {
            return Decimal::ONE;
        }

        let total = 16u32;
        let safe_total = total.saturating_sub(self.bombs as u32);

        if step > safe_total {
            return Decimal::ZERO;
        }

        let mut fair_multiplier = Decimal::ONE;
        for i in 0..step {
            let num = Decimal::from(total - i);
            let den = Decimal::from(safe_total - i);
            fair_multiplier *= num / den;
        }

        (fair_multiplier * super::RTP).round_dp(2)
    }

    pub fn current_multiplier(&self) -> Decimal {
        self.multiplier_at_step(self.opened_count())
    }

    fn next_multiplier(&self) -> Decimal {
        self.multiplier_at_step(self.opened_count() + 1)
    }

    pub fn current_win(&self) -> Decimal {
        if self.opened_count() == 0 {
            self.bet
        } else {
            (self.bet * self.current_multiplier()).round_dp(2)
        }
    }

    fn next_win(&self) -> Decimal {
        (self.bet * self.next_multiplier()).round_dp(2)
    }

    pub fn get_embed(&self) -> serenity::CreateEmbed {
        let opened = self.opened_count();
        let total_cells = 16u32;
        let closed_cells = total_cells - opened;
        let safe_total = total_cells.saturating_sub(self.bombs as u32);
        let safe_left = safe_total.saturating_sub(opened);

        let current_mult = self.current_multiplier();
        let next_mult = self.next_multiplier();
        let current_win = self.current_win();
        let next_win = self.next_win();

        let mut embed = serenity::CreateEmbed::new()
            .title("💣 ПОДПОЛЬНЫЙ САПЕР")
            .colour(serenity::Colour::from_rgb(255, 170, 0));

        if opened == 0 {
            embed = embed.description(format!(
                "💵 **Ставка:** `{}` бебр\n\
                 💣 **Мин на поле:** `{}` шт.\n\n\
                 Сделай первый ход! Нажми на любую кнопку `❓` ниже.",
                self.bet, self.bombs
            ));
        } else if opened == safe_total {
            embed = embed
                .colour(Colour::from_rgb(46, 204, 113))
                .description(format!(
                    "🏆 **ПОЛНАЯ ЗАЧИСТКА ПОЛЯ!**\n\n\
                     💵 **Ставка:** `{}` бебр\n\
                     💰 **Итоговый банк:** **`{}`** бебр (`x{}`)\n\n\
                     *Все безопасные клетки открыты! Забирай выигрыш!*",
                    self.bet, current_win, current_mult
                ));
        } else {
            let win_chance = (safe_left as f64 / closed_cells as f64) * 100.0;

            embed = embed
                .description(format!(
                    "💵 **Начальная ставка:** `{}` бебр\n\
                     💰 **Текущий банк:** **`{}`** бебр (`x{}`)\n\n\
                     ────────────────────────\n\
                     🎯 **Следующий ход:** `x{}` (+{} бебр)\n\
                     🎲 **Шанс на успех:** `{:.1}%` *(осталось {} 💎 из {} ячеек)*\n\
                     ────────────────────────\n\
                     *Забери банк или рискни ради большего!*",
                    self.bet,
                    current_win,
                    current_mult,
                    next_mult,
                    next_win - current_win,
                    win_chance,
                    safe_left,
                    closed_cells
                ))
                .footer(CreateEmbedFooter::new(
                    "⏱️ Авто-кэшаут через 5 минут бездействия",
                ));
        }

        embed
    }

    pub fn get_buttons(&self) -> Vec<CreateActionRow> {
        let mut rows = Vec::with_capacity(5);
        let current_win = self.current_win();
        let opened = self.opened_count();

        for row in 0..4 {
            let mut buttons = Vec::with_capacity(4);

            for col in 0..4 {
                let tile_idx = row * 4 + col;

                let button = if self.is_opened(tile_idx) {
                    CreateButton::new(format!("casino:mines:empty_{tile_idx}"))
                        .label("💎")
                        .disabled(true)
                } else {
                    let custom_id = self.to_custom_id(&MinesAction::Tile(tile_idx));
                    CreateButton::new(custom_id)
                        .style(ButtonStyle::Secondary)
                        .label("❓")
                };

                buttons.push(button);
            }

            rows.push(CreateActionRow::Buttons(buttons));
        }

        let can_cashout = opened > 0;
        let cashout_id = self.to_custom_id(&MinesAction::Cashout);

        let cashout_label = if can_cashout {
            format!("💰 Забрать {}", current_win)
        } else {
            "💰 Забрать".to_string()
        };

        let cashout_btn = CreateButton::new(cashout_id)
            .style(if can_cashout {
                ButtonStyle::Success
            } else {
                ButtonStyle::Secondary
            })
            .label(cashout_label)
            .disabled(!can_cashout);

        let bombs_info_btn = CreateButton::new("casino:mines:empty_99")
            .style(ButtonStyle::Danger)
            .label(format!("💣 {}", self.bombs))
            .disabled(true);

        let safe_left = (16 - self.bombs as u32).saturating_sub(opened);
        let safe_info_btn = CreateButton::new("casino:mines:empty_98")
            .style(ButtonStyle::Secondary)
            .label(format!("💎 {}", safe_left))
            .disabled(true);

        rows.push(CreateActionRow::Buttons(vec![
            cashout_btn,
            bombs_info_btn,
            safe_info_btn,
        ]));

        rows
    }

    fn get_bomb_positions(&self) -> Vec<u8> {
        let seed_num = u64::from_str_radix(&self.seed, 16).unwrap_or(0);
        let mut rng = StdRng::seed_from_u64(seed_num);

        let mut cells: Vec<u8> = (0..16).collect();
        cells.shuffle(&mut rng);

        cells.truncate(self.bombs as usize);
        cells
    }

    pub fn is_bomb(&self, tile: u8) -> bool {
        self.get_bomb_positions().contains(&tile)
    }

    pub fn open_tile(&mut self, index: u8) {
        self.mask |= 1 << index;
    }
}
