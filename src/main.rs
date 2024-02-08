use rand::prelude::*;
use std::{thread, time, io::{self, stdout}};
use crossterm::{
    style::{Color, Print, SetForegroundColor},
    ExecutableCommand,
    cursor,
    terminal,
    QueueableCommand,
};

const UPDATE_TIME: time::Duration = time::Duration::from_millis(50);

fn main() -> io::Result<()> {
    let (w, h) = terminal::size().unwrap();
    let mut th_rng = rand::thread_rng();
    
    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::Hide)?;

    let mut sentences: Vec<Sentence> = Vec::new();
       
    loop {
        let rand_sentence = th_rng.gen_range(0..=(w as i32 ));

        if !sentences.iter().any(|sen| sen.x == rand_sentence && sen.y - sen.length <= 10) {
            sentences.push(Sentence::new(rand_sentence));
        }

        sentences.retain(|sen| sen.y - sen.length <= h as i32);
        
        for sen in sentences.iter_mut() {
            sen.move_phrase().unwrap();
        }

        thread::sleep(UPDATE_TIME);
    }
}

struct Sentence {
    phrase: Vec<char>,
    length: i32,
    x: i32,
    y: i32,
    th_rng: ThreadRng
}

impl Sentence {
    fn new(x: i32) -> Sentence {
        let mut th_rng = rand::thread_rng();
        let length = th_rng.gen_range(5..28i32);
        let mut phrase = Vec::with_capacity(length as usize);

        for _ in 0..length {
            phrase.push(Self::random_char(&mut th_rng));
        }

        Sentence {
            phrase,
            length,
            x,
            y: 0,
            th_rng
        }
    }

    fn move_phrase(&mut self) -> io::Result<()> {
        
        let mut stdou = stdout();
        stdou
            .execute(cursor::MoveTo(self.x as u16, self.y as u16))?
            .execute(SetForegroundColor(Color::Rgb {r: 255, g: 255, b: 255}))?
            .execute(Print(self.phrase[0]))?;
        
        for i in 1..(self.length) {
            if self.y - i < 0 {break;}
            stdou
                .queue(cursor::MoveTo(self.x as u16, (self.y - i) as u16))?
                .queue(SetForegroundColor(Color::Rgb {
                    r: 0,
                    g: (((self.length as f32 - i as f32 + 1.0) /self.length as f32) * 255.0) as u8, 
                    b: 0
                }))?
                .queue(Print(self.phrase[i as usize]))?;
        }


        if self.y >= self.length {
            stdou
                .execute(cursor::MoveTo(self.x as u16, (self.y - self.length) as u16))?
                .execute(Print(" "))?;
               
        } 

        let ch = Self::random_char(&mut self.th_rng);
        self.y += 1;
        self.phrase.insert(0, ch);
        self.phrase.pop();
        Ok(())
    }

    fn random_char(th_rng: &mut ThreadRng) -> char {
        th_rng.gen_range('<'..='z')
    }
}
