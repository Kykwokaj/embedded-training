//! Complete the two TODOs in this file.

use embassy_stm32::{
    Peripherals, bind_interrupts, peripherals,
    can::Can,
    gpio::{Flex, Level, Output, Speed},
    usart::{self, BufferedUart},
};
use static_cell::StaticCell;

use crate::swd::Swd;

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // TODO 1: Pick two exposed 3.3 V GPIOs and pass them to Swd::new.
    // Use Output for SWCLK (initially low), and Flex for bidirectional SWDIO.
    let swd = Some(Swd::new(
        Output::new(p.PA00.degrade(), Level::Low, Speed::Low),
        Flex::new(p.PA02.degrade()),
    ));


    // TODO 2: Configure a buffered UART at 115200 baud, 8N1.
    // Identify the peripheral, RX/TX pins and interrupt from the schematic.
    // Bind the interrupt and provide static RX and TX buffers.
    let uart = crate::board::uart(UART_BAUD, p.USART7, p.PE07, p.PB08, p.IRQ_USART7);
    static TX_BUF: StaticCell<[u8; 256]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; 256]> = StaticCell::new();
    let mut cfg = usart::Config::default(); // default = 8N1
    cfg.baudrate = UART_BAUD;
    let uart = BufferedUart::new(
        p.UART7,
        p.PE7, // RX
        p.PE8, // TX
        TX_BUF.init([0; 256]),
        RX_BUF.init([0; 256]),
        UartIrqs,
        cfg,
    )
    .ok();

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart }, swd)
}
