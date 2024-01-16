slint::include_modules!();

const ALK_CONST: f64 = 0.132;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_calc_volym(move |string| {
        let ui = ui_handle.unwrap();
        let og: f64 = string.trim().parse().unwrap();
        let vol: f64 = og * ALK_CONST;
        let result: String = format!("{:.2} Volym %" ,vol);
        ui.set_results(result.into());
    });

    ui.run()
}
