mod headlines;

pub use headlines::{Headlines, Msg, NewsCardData, PADDING};
use eframe::App;
use newsapi::NewsAPI;
use std::sync::mpsc::Sender;
use eframe::egui::{
    RichText,
    TopBottomPanel,
    TextStyle,
};


impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        let scale_value:f32 = headlines::read_from_scale_value();
        ctx.set_pixels_per_point(scale_value);
        if self.config.dark_mode {
            ctx.set_visuals(eframe::egui::Visuals::dark());
        } else {
            ctx.set_visuals(eframe::egui::Visuals::light());
        }

        if !self.api_key_initialized || headlines::SHOW_CONFIG.lock().unwrap().clone(){
            self.render_config(ctx);
        } else {
            self.preload_articles();
            self.render_top_panel(ctx, frame);
            eframe::egui::CentralPanel::default().show(ctx, |ui| {
                // render_header(ui);
                eframe::egui::containers::ScrollArea::new([false, true])
                    .auto_shrink([false, false])
                    .always_show_scroll(false)
                    .show(ui, |ui| self.render_news_cards(ui));
                    if headlines::SHOW_FOOTER.lock().unwrap().clone() {
                        render_footer(ctx);
                    }
                });
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "headlines", &self.config);
    }
}

fn fetch_news(api_key: &str, news_tx: &Sender<NewsCardData>) {
    let response = NewsAPI::new(&api_key).fetch();
    if let Ok(response) = response {
        tracing::info!("Fetched!");
        let response_articles = response.articles();
        for a in response_articles.iter() {
            let news = NewsCardData {
                title: a.title().to_string(),
                url: a.url().to_string(),
                description: a.description().map(|s| s.to_string()).unwrap_or("...".to_string())
            };
            if let Err(e) = news_tx.send(news) {
                tracing::error!("Error sending data: {}", e);
            }
            // articles.push(news);
        }
    } else {
        tracing::error!("Could not fetch articles: {:?}", response);
    }
}

fn render_footer(ctx: &eframe::egui::Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.label(RichText::new("API Source: newsapi.org").monospace());
            ui.hyperlink_to(
                RichText::new("username/reponame").text_style(TextStyle::Monospace), 
                "https://github.com/emilk/egui"
            );
            ui.add_space(10.);
        });
    });
}
