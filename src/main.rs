#![no_std]
#![no_main]

pub mod registers;
pub mod gpio;
pub mod systick;
pub mod drivers;

use core::ptr;
use cortex_m_rt::entry;
use panic_halt as _;

// ===== АДРЕСА РЕГИСТРОВ =====
const RCC_BASE: u32 = 0x40021000;
const RCC_APB2EN_OFFSET: u32 = 0x18;

// PORTA (для реле PA5)
const GPIOA_BASE: u32 = 0x40010800;
const GPIOA_CFGLR_OFFSET: u32 = 0x00;
const GPIOA_SCR_OFFSET: u32 = 0x10;
const GPIOA_CLR_OFFSET: u32 = 0x14;

// PORTD (для светодиодов PD13, PD14, PD15)
const GPIOD_BASE: u32 = 0x40011400;
const GPIOD_CFGHR_OFFSET: u32 = 0x04;  // для пинов 8-15
const GPIOD_SCR_OFFSET: u32 = 0x10;
const GPIOD_CLR_OFFSET: u32 = 0x14;

// Пины
const RELAY_PIN: u32 = 5;   // PA5
const LED2_PIN: u32 = 13;   // PD13 (красный)
const LED3_PIN: u32 = 14;   // PD14 (жёлтый)
const LED4_PIN: u32 = 15;   // PD15 (зелёный)

// --- ЗАДЕРЖКА ---
#[inline(never)]
fn delay(count: u32) {
    for _ in 0..count {
        unsafe {
            ptr::read_volatile(&count);
        }
    }
}

#[entry]
fn main() -> ! {
    // ===== 1. Включаем тактирование для PORTA и PORTD =====
    unsafe {
        let rcc_apb2en = (RCC_BASE + RCC_APB2EN_OFFSET) as *mut u32;
        let current = rcc_apb2en.read_volatile();
        // Бит 2 = IOPAEN (PORTA), Бит 5 = IOPDEN (PORTD)
        rcc_apb2en.write_volatile(current | (1 << 2) | (1 << 5));
    }

    // ===== 2. Настраиваем PA5 как выход (реле) =====
    unsafe {
        let cfglr = (GPIOA_BASE + GPIOA_CFGLR_OFFSET) as *mut u32;
        let current_val = cfglr.read_volatile();
        
        // PA5 — пин 5 → биты 20-23 (5 * 4 = 20)
        let cleared = current_val & !(0b1111 << 20);
        let new_val = cleared | (0b0011 << 20);
        cfglr.write_volatile(new_val);
    }

    // ===== 3. Настраиваем PD13, PD14, PD15 как выходы (светодиоды) =====
    unsafe {
        let cfghr = (GPIOD_BASE + GPIOD_CFGHR_OFFSET) as *mut u32;
        let current_val = cfghr.read_volatile();
        
        // PD13 — пин 13 → биты 20-23 ((13-8)*4 = 20)
        let val = (current_val & !(0b1111 << 20)) | (0b0011 << 20);
        // PD14 — пин 14 → биты 24-27
        let val = (val & !(0b1111 << 24)) | (0b0011 << 24);
        // PD15 — пин 15 → биты 28-31
        let val = (val & !(0b1111 << 28)) | (0b0011 << 28);
        
        cfghr.write_volatile(val);
    }

    // ===== 4. Основной цикл =====
    loop {
        // ===== ЭТАП 1: Включаем реле, зажигаем LED2 (красный) =====
        unsafe {
            // Включаем реле (0 = включено)
            let clr = (GPIOA_BASE + GPIOA_CLR_OFFSET) as *mut u32;
            clr.write_volatile(1 << RELAY_PIN);
            
            // Зажигаем LED2 (0 = горит)
            let led_clr = (GPIOD_BASE + GPIOD_CLR_OFFSET) as *mut u32;
            led_clr.write_volatile(1 << LED2_PIN);
            
            // Гасим остальные светодиоды
            let led_scr = (GPIOD_BASE + GPIOD_SCR_OFFSET) as *mut u32;
            led_scr.write_volatile((1 << LED3_PIN) | (1 << LED4_PIN));
        }
        delay(2_000_000);  // пауза ~1 секунда
        
        // ===== ЭТАП 2: Реле включено, зажигаем LED3 (жёлтый) =====
        unsafe {
            // Реле оставляем включенным
            // Зажигаем LED3, гасим LED2 и LED4
            let led_clr = (GPIOD_BASE + GPIOD_CLR_OFFSET) as *mut u32;
            led_clr.write_volatile(1 << LED3_PIN);
            
            let led_scr = (GPIOD_BASE + GPIOD_SCR_OFFSET) as *mut u32;
            led_scr.write_volatile((1 << LED2_PIN) | (1 << LED4_PIN));
        }
        delay(2_000_000);
        
        // ===== ЭТАП 3: Реле включено, зажигаем LED4 (зелёный) =====
        unsafe {
            let led_clr = (GPIOD_BASE + GPIOD_CLR_OFFSET) as *mut u32;
            led_clr.write_volatile(1 << LED4_PIN);
            
            let led_scr = (GPIOD_BASE + GPIOD_SCR_OFFSET) as *mut u32;
            led_scr.write_volatile((1 << LED2_PIN) | (1 << LED3_PIN));
        }
        delay(2_000_000);
        
        // ===== ЭТАП 4: ВЫКЛЮЧАЕМ реле, все светодиоды горят =====
        unsafe {
            // Выключаем реле (1 = выключено)
            let scr = (GPIOA_BASE + GPIOA_SCR_OFFSET) as *mut u32;
            scr.write_volatile(1 << RELAY_PIN);
            
            // Все светодиоды зажигаем
            let led_clr = (GPIOD_BASE + GPIOD_CLR_OFFSET) as *mut u32;
            led_clr.write_volatile((1 << LED2_PIN) | (1 << LED3_PIN) | (1 << LED4_PIN));
        }
        delay(2_000_000);
        
        // ===== ЭТАП 5: Все выключаем (пауза перед повтором) =====
        unsafe {
            let led_scr = (GPIOD_BASE + GPIOD_SCR_OFFSET) as *mut u32;
            led_scr.write_volatile((1 << LED2_PIN) | (1 << LED3_PIN) | (1 << LED4_PIN));
        }
        delay(2_000_000);
    }
}