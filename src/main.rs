use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk::StackTransitionType::SlideLeftRight;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(360)
        .default_height(360)
        .build();

    let container = gtk::Box::new(gtk::Orientation::Vertical, 100);
    window.set_child(Some(&container));

    let stack = gtk::Stack::new();

    stack.set_transition_type(SlideLeftRight);
    stack.set_transition_duration(200);

    let page_1_label = gtk::Label::new(Some("Page 1"));
    stack.add_titled(&page_1_label, Option::<&str>::None, "Page 1");

    let page_2_label = gtk::Label::new(Some("Page 2"));
    stack.add_titled(&page_2_label, Option::<&str>::None, "Page 2");

    // Create a button with label and margins
    let page_3_button = gtk::Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    stack.add_titled(&page_3_button, Option::<&str>::None, "Button");

    // Connect to "clicked" signal of `button`
    page_3_button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    let stack_switcher = gtk::StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));

    container.append(&stack_switcher);
    container.append(&stack);

    window.present();

}
