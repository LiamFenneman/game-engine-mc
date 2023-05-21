use std::collections::BTreeMap;
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text};
use winit::dpi::PhysicalSize;

pub trait DrawText {
    fn name(&self) -> &'static str;
    fn priority(&self) -> u8;
    fn text(&self) -> String;
}

#[derive(Debug)]
pub struct TextRenderer {
    lines: BTreeMap<TextEntry, String>,
    size: PhysicalSize<u32>,
    glyph_brush: wgpu_glyph::GlyphBrush<()>,
}

impl TextRenderer {
    /// Create a new `TextRenderer`.
    ///
    /// # Panics
    /// Panics if the font file is not found.
    #[must_use]
    pub fn new(
        title: &str,
        screen_size: impl Into<PhysicalSize<u32>>,
        device: &wgpu::Device,
        render_format: wgpu::TextureFormat,
    ) -> Self {
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../../assets/fonts/IBMPlexMono-Regular.ttf"
        ))
        .unwrap();

        let glyph_brush = GlyphBrushBuilder::using_font(font).build(device, render_format);

        let mut lines: BTreeMap<TextEntry, String> = BTreeMap::new();
        lines.insert(("title", u8::MAX).into(), title.to_string());

        return Self {
            lines,
            size: screen_size.into(),
            glyph_brush,
        };
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        staging_belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) {
        #[allow(clippy::cast_precision_loss, reason = "there is no other way")]
        self.glyph_brush.queue(Section {
            screen_position: (10.0, 10.0),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![Text::new(&self.text())
                .with_color([1.0, 1.0, 1.0, 1.0])
                .with_scale(20.0)],
            ..Section::default()
        });

        self.glyph_brush
            .draw_queued(
                device,
                staging_belt,
                encoder,
                view,
                self.size.width,
                self.size.height,
            )
            .expect("Draw queued");
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
    }

    fn text(&self) -> String {
        return self.lines.values().fold(String::new(), |mut acc, line| {
            acc.push_str(line);
            acc.push('\n');
            return acc;
        });
    }

    pub fn add_entry<T: DrawText>(&mut self, t: &T) {
        self.lines.insert((t.name(), t.priority()).into(), t.text());
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TextEntry {
    name: &'static str,
    priority: u8,
}

impl PartialOrd for TextEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for TextEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.priority.cmp(&self.priority);
    }
}

impl From<(&'static str, u8)> for TextEntry {
    fn from(value: (&'static str, u8)) -> Self {
        return Self {
            name: value.0,
            priority: value.1,
        };
    }
}
