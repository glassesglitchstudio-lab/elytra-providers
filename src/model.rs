//! Model tanımı — bir sağlayıcının sunduğu tek bir model.

/// Tek bir model tanımı (katalog verisi — statik).
///
/// Alanlar `&'static str` çünkü katalog derleme zamanında sabittir;
/// kullanıcı seçimi config'e `String` olarak yazılır.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Model {
    /// API'de kullanılan model kimliği (örn. `"deepseek-chat"`)
    pub id: &'static str,
    /// İnsan okunur ad
    pub name: &'static str,
    /// Bağlam penceresi (token)
    pub context: u32,
    /// Ücretsiz katmanda kullanılabiliyor mu?
    pub free: bool,
    /// Akıl yürütme (reasoning) modeli mi?
    pub reasoning: bool,
    /// Kod görevleri için öneriliyor mu?
    pub coding: bool,
}

impl Model {
    /// Yalnız zorunlu alanlarla model üretir (diğer bayraklar `false`).
    pub const fn new(id: &'static str, name: &'static str, context: u32, free: bool) -> Self {
        Self { id, name, context, free, reasoning: false, coding: false }
    }

    /// Kod modeli işaretler.
    pub const fn coding(mut self) -> Self {
        self.coding = true;
        self
    }

    /// Akıl yürütme modeli işaretler.
    pub const fn reasoning(mut self) -> Self {
        self.reasoning = true;
        self
    }

    /// Bağlam penceresini insan okunur biçimde döner (örn. `"128K"`, `"1.0M"`).
    pub fn context_label(&self) -> String {
        if self.context >= 1_000_000 {
            format!("{:.1}M", self.context as f64 / 1_000_000.0)
        } else if self.context >= 1_000 {
            format!("{}K", self.context / 1_000)
        } else {
            self.context.to_string()
        }
    }
}
