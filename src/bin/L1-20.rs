struct MicroUSBCharger;

impl MicroUSBCharger {
    fn charge_via_micro_usb(&self) {
        println!("Зарядка через microUSB...");
    }
}

trait USBCharger {
    fn charge_via_usb_c(&self);
}

struct MicroUSBToUSBAdapter {
    micro_usb_charger: MicroUSBCharger,
}

impl MicroUSBToUSBAdapter {
    fn new(micro_usb_charger: MicroUSBCharger) -> Self {
        MicroUSBToUSBAdapter { micro_usb_charger }
    }
}

impl USBCharger for MicroUSBToUSBAdapter {
    fn charge_via_usb_c(&self) {
        self.micro_usb_charger.charge_via_micro_usb();
    }
}

fn main() {
    let old_charger = MicroUSBCharger;

    let adapter = MicroUSBToUSBAdapter::new(old_charger);

    adapter.charge_via_usb_c();
}
