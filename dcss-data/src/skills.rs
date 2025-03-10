use crate::CrawlData;
use rustc_hash::FxHashMap;
use serde_json::Value;

#[derive(Debug)]
pub(crate) struct Skill {
    pub(crate) id: String,
    pub(crate) selected: i32,
    pub(crate) _level: f64,
    pub(crate) _aptitude: i32,
}

#[derive(Debug)]
pub(crate) struct Skills {
    pub(crate) skills: FxHashMap<String, Skill>,
    pub(crate) skill_training: Vec<(String, i32)>,
    pub(crate) skill_menu_items: FxHashMap<u32, String>,
}

impl Skill {
    pub(crate) fn new(id: String, selected: i32, _level: f64, _aptitude: i32) -> Self {
        Self {
            id,
            selected,
            _level,
            _aptitude,
        }
    }
}

impl Skills {
    pub(crate) fn new() -> Self {
        Self {
            skills: FxHashMap::default(),
            skill_training: vec![],
            skill_menu_items: FxHashMap::default(),
        }
    }

    #[allow(clippy::assigning_clones)] // Bug? line = line[][].to_owned() can't be converted
    pub(crate) fn process_skills(&mut self) {
        'num_lines: for i in 0..25 {
            // Ignore empty lines or non-provided data
            if !self.skill_menu_items.contains_key(&i) {
                continue;
            }
            if self.skill_menu_items[&i].is_empty() {
                continue;
            }

            // Create line and add two character to make first column match
            // the second column.
            let mut line = self.skill_menu_items[&i].clone();
            line.push(' ');
            line.push(' ');

            // Replace annoying text in name (and html);
            line = line.replace(" &amp; ", " & ");

            // Find start of name (strip unimportant)
            let line_start = line[0..line.len()].find('>');
            if line_start.is_none() {
                continue; // not a skill (skip)
            }
            line = line[line_start.unwrap() + 1..line.len()].to_owned();

            for _ in 0..2 {
                // Empty, ignore
                if line.is_empty() {
                    continue;
                }

                // Find ID and if it's selected
                let id = line.chars().next().unwrap().to_string();
                let selected_text: String = line.chars().nth(2).unwrap().to_string();
                let selected = match &selected_text[..] {
                    "-" => 0,
                    "+" => 1,
                    "*" => 2,
                    _ => continue 'num_lines, // not a skill (skip)
                };

                // Get the name of the skill
                line = line[4..line.len()].to_owned();
                let name_end = line.find("  ").unwrap();
                let name = line[0..name_end].to_owned();
                line = line[name_end..line.len()].trim_start().to_owned();

                // Get level
                let level_end = line.find(' ').unwrap();
                let level = line[0..level_end].to_owned().parse::<f64>().unwrap();

                // Get cost
                let end = line.find('>').unwrap();
                line = line[end + 1..line.len()].to_owned();
                let end: usize = line.find('>').unwrap();
                line = line[end + 1..line.len()].to_owned();
                let cost_end = line.find(' ').unwrap();
                let _cost = line[0..cost_end].to_owned().parse::<f64>().unwrap();

                // Get aptitude
                let end = line.find('>').unwrap();
                line = line[end + 1..line.len()].to_owned();
                let end = line.find('>').unwrap();
                line = line[end + 1..line.len()].to_owned();
                let apt_end = line.find(' ').unwrap();
                let aptitude = line[0..apt_end].to_owned().parse::<i32>().unwrap();

                // Prepare for second skill
                let end = line.find("  ").unwrap();
                line = line[end + 1..line.len()].to_owned().trim_start().to_owned();
                if line.starts_with('<') {
                    let end = line.find('>').unwrap();
                    line = line[end + 1..line.len()].to_owned().trim_start().to_owned();
                    let end = line.find('>').unwrap();
                    line = line[end + 1..line.len()].to_owned().trim_start().to_owned();
                }

                self.skills
                    .insert(name, Skill::new(id, selected, level, aptitude));
            }
        }
    }
}

impl CrawlData {
    pub fn process_skills(&mut self, menu_items: &Value) {
        let skills_obj = menu_items.as_object().unwrap();

        // Empty -- ignore
        if !skills_obj.contains_key("lines") {
            return;
        }

        // Game only sends changed lines -- so some option changes may not re-send
        // all the lines. Store them and update if new info is sent.
        let lines_obj = skills_obj["lines"].as_object().unwrap();
        let mut has_some_data = false;
        for i in 0..25 {
            if lines_obj.contains_key(&(i.to_string())) {
                // Verify at least one line has data
                if !lines_obj[&(i.to_string())]
                    .as_str()
                    .unwrap()
                    .to_owned()
                    .is_empty()
                {
                    has_some_data = true;
                }

                self.skills
                    .skill_menu_items
                    .insert(i, lines_obj[&(i.to_string())].as_str().unwrap().to_owned());
            }
        }
        // If none found, it's because the game is simply blanking everything when the menu is closed.
        if !has_some_data {
            return;
        }

        // Identify menu options
        let option1 = self.skills.skill_menu_items[&21].to_owned();
        let option2 = self.skills.skill_menu_items[&22].to_owned();
        let option3 = self.skills.skill_menu_items[&23].to_owned();
        let options = [option1, option2, option3].join("\n");
        let (mode, scope, view) = skills_options(options);

        if self.correct_skills_mode(mode, scope, view) {
            self.skills.process_skills();

            self.set_skills_training();
        }
    }

    pub(crate) fn set_skills_training(&mut self) {
        for (skill_name, skill_data) in &self.skills.skills {
            let mut found = false;
            for (to_train_name, to_train_selected) in &self.skills.skill_training {
                if to_train_name == skill_name {
                    found = true;
                    if &skill_data.selected != to_train_selected {
                        self.menus.add_menu(
                            vec!["m"],
                            &skill_data.id,
                            &skill_data.id,
                            false,
                            true,
                            false,
                            false,
                            "",
                            "txt",
                        );
                        return;
                    }
                }
            }
            if !found && skill_data.selected != 0 {
                self.menus.add_menu(
                    vec!["m"],
                    &skill_data.id,
                    &skill_data.id,
                    false,
                    true,
                    false,
                    false,
                    "",
                    "txt",
                );
                return;
            }
        }
    }

    fn correct_skills_mode(&mut self, mode: String, scope: String, view: String) -> bool {
        if mode != "manual" {
            self.menus
                .add_menu(vec!["m"], "/", "/", false, true, false, false, "", "txt");
            return false;
        }

        if scope != "all" {
            self.menus
                .add_menu(vec!["m"], "*", "*", false, true, false, false, "", "txt");
            return false;
        }

        if view != "cost" {
            self.menus
                .add_menu(vec!["m"], "!", "!", false, true, false, false, "", "txt");
            return false;
        }

        true
    }

    pub fn set_skill_training(&mut self, training: Vec<(String, i32)>) {
        self.skills.skill_training = training;
    }
}

pub(crate) fn skills_options(options: String) -> (String, String, String) {
    // Mode type (auto or manual)
    let mut mode = "manual";
    if options.contains("<span class=\"fg15 bg0\">auto</span>") {
        mode = "auto";
    }

    // Scope of skills (useful or all)
    let mut scope = "all";
    if options.contains("<span class=\"fg15 bg0\">useful</span>") {
        scope = "useful";
    }

    // Scope of skills (useful or all)
    let mut view = "targets";
    if options.contains("<span class=\"fg15 bg0\">training</span>") {
        view = "training";
    } else if options.contains("<span class=\"fg15 bg0\">cost</span>") {
        view = "cost";
    }

    (mode.to_owned(), scope.to_owned(), view.to_owned())
}
