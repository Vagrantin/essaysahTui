use std::io::{self,Result};
use ratatui::{
    prelude::*,
    crossterm::event::{self, KeyCode, KeyEventKind},
    widgets::*,
    DefaultTerminal,
};

mod app;

// when scrolling, make sure we don't scroll past the last item in the Vector
// fn maxlength()
// from example_height
//
// Get the height of all items combined
//fn example_height() -> u16 {
//    EXAMPLE_DATA
//        .iter()
//        .map(|(desc, _)| get_description_height(desc) + 4)
//        .sum()
//}
#[allow(unused_doc_comments)]
fn run_render(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = app::App::new();
    loop {
        //Get the height of all items in the Vector
        //let height = example_height();
        //Check from the heigh if we need the scroll - We probably want to have it all the time.
        //let scrollbar_needed = self.scroll_offset != 0 || height > area.height;
        //let content_area = if scrollbar_needed {
        //    Rect {
        //        width: demo_area.width - 1,
        //        ..demo_area
        //    }
        //} else {
        //    demo_area
        //};
        //This is where we are building the scrollbar
        //
        //This is probably the part that we want
        //if scrollbar_needed {
        //    let area = area.intersection(buf.area);
        //    let mut state = ScrollbarState::new(max_scroll_offset() as usize)
        //        .position(self.scroll_offset as usize);
        //    Scrollbar::new(ScrollbarOrientation::VerticalRight).render(area, buf, &mut state);
        ///    ****
        ///    I think there is something to do like above to re-render at some point.
        ///    ***
        //}
        terminal.draw(|frame| {
            let chunks = Layout::vertical([
                Constraint::Min(1),
                Constraint::Length(5),
            ]);
            let [text, debug] = chunks.areas(frame.area());
            let items: Vec<ListItem> = app
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    if i == app.selected {
                        ListItem::new::<Line<'_>>(item.clone()).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new::<Line<'_>>(item.clone())
                    }
                })
                .collect();
            let list = List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL));
            app.vertical_scroll_state = app.vertical_scroll_state.content_length(list.len());
            frame.render_widget(list, text);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                text,
                &mut app.vertical_scroll_state,
            );
            let status = format!("{}", &app.status_message);

            let debug_message = Paragraph::new(status)
                .block(Block::bordered().title("debug"));
            frame.render_widget(debug_message, debug);

        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                terminal.clear()?;
                return Ok(());
            } else {
             match key.code {
                 KeyCode::Up => app.move_up(),
                 KeyCode::Char('t') => app.move_up(),
                 //KeyCode::Down => app.move_down(),
                 KeyCode::Char('s') | KeyCode::Down =>{
                     app.move_down();
                     app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                     app.vertical_scroll_state =
                         app.vertical_scroll_state.position(app.vertical_scroll);
                 },
                 KeyCode::Enter => {
                    app.tmux_session()?;
                    terminal.clear()?;
             },
                 _ => {}
             }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run_render(terminal);
    ratatui::restore();
    app_result
}
