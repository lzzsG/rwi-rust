use eframe::egui;
use rand::Rng;
use std::cmp::Ordering;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Guess the Number Game",
        options,
        Box::new(|_cc| Ok(Box::new(GuessingGame::new()))),
    )
}

#[derive(Default)]
struct GuessingGame {
    secret_number: u32,
    guess: String,
    message: String,
    game_over: bool,
}

impl GuessingGame {
    fn new() -> Self {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        Self {
            secret_number,
            guess: String::new(),
            message: String::from("Please input your guess."),
            game_over: false,
        }
    }

    fn reset(&mut self) {
        self.secret_number = rand::thread_rng().gen_range(1..=100);
        self.guess.clear();
        self.message = String::from("Please input your guess.");
        self.game_over = false;
    }
}

impl eframe::App for GuessingGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guess the Number Game");

            ui.label(&self.message);

            if self.game_over {
                if ui.button("Play Again").clicked() {
                    self.reset();
                }
            } else {
                ui.horizontal(|ui| {
                    ui.label("Your guess: ");
                    ui.text_edit_singleline(&mut self.guess);
                });

                if ui.button("Submit").clicked() {
                    match self.guess.trim().parse::<u32>() {
                        Ok(num) => match num.cmp(&self.secret_number) {
                            Ordering::Less => self.message = String::from("Too small!"),
                            Ordering::Greater => self.message = String::from("Too big!"),
                            Ordering::Equal => {
                                self.message = String::from(
                                    "You win! Press 'Play Again' to start a new game.",
                                );
                                self.game_over = true;
                            }
                        },
                        Err(_) => {
                            self.message = String::from("Please enter a valid number!");
                        }
                    }
                    self.guess.clear(); // Clear the input after submission
                }
            }
        });
    }
}
