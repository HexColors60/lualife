use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Resource, Debug, Clone, Default)]
pub struct Profiler {
    pub frame_start: Option<Instant>,
    pub sections: HashMap<String, ProfileSection>,
    pub enabled: bool,
    pub frame_count: u64,
    pub total_frame_time_us: u64,
    pub max_sections: usize,
}

#[derive(Debug, Clone)]
pub struct ProfileSection {
    pub name: String,
    pub start: Option<Instant>,
    pub total_us: u64,
    pub call_count: u64,
    pub max_us: u64,
    pub min_us: u64,
}

impl ProfileSection {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: None,
            total_us: 0,
            call_count: 0,
            max_us: 0,
            min_us: u64::MAX,
        }
    }

    pub fn average_us(&self) -> f64 {
        if self.call_count > 0 {
            self.total_us as f64 / self.call_count as f64
        } else {
            0.0
        }
    }
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            frame_start: None,
            sections: HashMap::new(),
            enabled: true,
            frame_count: 0,
            total_frame_time_us: 0,
            max_sections: 100,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn begin_frame(&mut self) {
        if !self.enabled {
            return;
        }
        self.frame_start = Some(Instant::now());
        self.frame_count += 1;
    }

    pub fn end_frame(&mut self) {
        if !self.enabled {
            return;
        }
        if let Some(start) = self.frame_start {
            let elapsed = start.elapsed().as_micros() as u64;
            self.total_frame_time_us += elapsed;
        }
        self.frame_start = None;
    }

    pub fn begin_section(&mut self, name: &str) {
        if !self.enabled {
            return;
        }
        let section = self
            .sections
            .entry(name.to_string())
            .or_insert_with(|| ProfileSection::new(name));
        section.start = Some(Instant::now());
    }

    pub fn end_section(&mut self, name: &str) {
        if !self.enabled {
            return;
        }
        if let Some(section) = self.sections.get_mut(name) {
            if let Some(start) = section.start {
                let elapsed = start.elapsed().as_micros() as u64;
                section.total_us += elapsed;
                section.call_count += 1;
                section.max_us = section.max_us.max(elapsed);
                section.min_us = section.min_us.min(elapsed);
            }
            section.start = None;
        }
    }

    pub fn get_section(&self, name: &str) -> Option<&ProfileSection> {
        self.sections.get(name)
    }

    pub fn get_sorted_sections(&self) -> Vec<(&String, &ProfileSection)> {
        let mut sections: Vec<_> = self.sections.iter().collect();
        sections.sort_by(|a, b| b.1.total_us.cmp(&a.1.total_us));
        sections
    }

    pub fn average_frame_time_ms(&self) -> f64 {
        if self.frame_count > 0 {
            (self.total_frame_time_us as f64 / self.frame_count as f64) / 1000.0
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        self.sections.clear();
        self.frame_count = 0;
        self.total_frame_time_us = 0;
    }

    pub fn report(&self) -> ProfilerReport {
        let mut sections = Vec::new();
        for (name, section) in &self.sections {
            sections.push(SectionReport {
                name: name.clone(),
                total_us: section.total_us,
                call_count: section.call_count,
                average_us: section.average_us(),
                max_us: section.max_us,
                min_us: if section.min_us == u64::MAX {
                    0
                } else {
                    section.min_us
                },
            });
        }
        sections.sort_by(|a, b| b.total_us.cmp(&a.total_us));

        ProfilerReport {
            frame_count: self.frame_count,
            average_frame_time_ms: self.average_frame_time_ms(),
            sections,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProfilerReport {
    pub frame_count: u64,
    pub average_frame_time_ms: f64,
    pub sections: Vec<SectionReport>,
}

#[derive(Debug, Clone)]
pub struct SectionReport {
    pub name: String,
    pub total_us: u64,
    pub call_count: u64,
    pub average_us: f64,
    pub max_us: u64,
    pub min_us: u64,
}

#[derive(Component)]
pub struct ProfilerDisplay;

pub fn profiler_frame_system(mut profiler: ResMut<Profiler>) {
    profiler.end_frame();
    profiler.begin_frame();
}

pub fn profiler_report_system(
    profiler: Res<Profiler>,
    mut query: Query<&mut Text, With<ProfilerDisplay>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F11) {
        let report = profiler.report();
        let mut text = String::new();
        text.push_str("=== Performance Report ===\n");
        text.push_str(&format!(
            "Frames: {} | Avg: {:.2}ms\n\n",
            report.frame_count, report.average_frame_time_ms
        ));

        for section in report.sections.iter().take(10) {
            text.push_str(&format!(
                "{}: {:.2}us avg ({}/{} calls)\n",
                section.name, section.average_us, section.max_us, section.min_us
            ));
        }

        for mut t in query.iter_mut() {
            t.sections[0].value = text.clone();
        }
    }
}

pub struct ProfilingPlugin;

impl Plugin for ProfilingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Profiler>()
            .add_systems(First, profiler_frame_system)
            .add_systems(Last, profiler_report_system);
    }
}

pub struct ProfileGuard<'a> {
    profiler: &'a mut Profiler,
    name: String,
}

impl<'a> ProfileGuard<'a> {
    pub fn new(profiler: &'a mut Profiler, name: &str) -> Self {
        profiler.begin_section(name);
        Self {
            profiler,
            name: name.to_string(),
        }
    }
}

impl Drop for ProfileGuard<'_> {
    fn drop(&mut self) {
        self.profiler.end_section(&self.name);
    }
}

#[macro_export]
macro_rules! profile_section {
    ($profiler:expr, $name:expr, $block:block) => {{
        $profiler.begin_section($name);
        let result = $block;
        $profiler.end_section($name);
        result
    }};
}
