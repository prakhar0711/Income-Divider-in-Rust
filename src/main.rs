slint::include_modules!();

const SAVPER:f64=0.40;
const INVSPER:f64=0.30;
const EXPPER:f64=0.25;
const SELFPER:f64=0.05;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        let num:f64=string.trim().parse().unwrap();
        let savings:f64=num * SAVPER;
        let invest:f64=num * INVSPER;
        let expense:f64=num * EXPPER;
        let self_expense:f64=num * SELFPER;
        let result=format!("Savings : {:.2}\nInvestment : {:.2}\nExpense : {:.2}\nSelf Expense : {:.2}",savings,invest,expense,self_expense);
        ui.set_results(result.into());
    });

    ui.run()
}
