//! Sağlayıcı tanımı — bir LLM servisi (API uç noktası + kimlik doğrulama).

use crate::model::Model;

/// Sağlayıcının konuştuğu API protokolü.
///
/// Çoğu sağlayıcı OpenAI uyumludur; Anthropic ve Google kendi biçimlerini
/// kullanır — istemci katmanı bu ayrımı bilerek istek kurar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiKind {
    /// `POST {base}/chat/completions` — OpenAI biçimi (en yaygın)
    OpenAiCompatible,
    /// `POST {base}/messages` — Anthropic Mesajlar API'si
    Anthropic,
    /// Google Generative Language API (OpenAI uyumlu köprü de kullanılabilir)
    Google,
}

/// Bir sağlayıcı tanımı (katalog verisi — statik).
#[derive(Debug, Clone, Copy)]
pub struct Provider {
    /// Kısa kimlik (config'de saklanır, örn. `"groq"`)
    pub id: &'static str,
    /// İnsan okunur ad
    pub name: &'static str,
    /// Tek satır ipucu (ücretsiz kota, güçlü yön)
    pub hint: &'static str,
    /// API kök adresi (sonda `/` YOK)
    pub base_url: &'static str,
    /// Anahtarın okunacağı ortam değişkeni
    pub env_key: &'static str,
    /// API protokolü
    pub kind: ApiKind,
    /// Ücretsiz katman var mı?
    pub is_free_tier: bool,
    /// Anahtarın alındığı adres
    pub signup: &'static str,
    /// Önerilen başlangıç modelleri
    pub models: &'static [Model],
}

impl Provider {
    /// `/chat/completions` tam adresini üretir (OpenAI uyumlu sağlayıcılar için).
    pub fn chat_url(&self) -> String {
        format!("{}/chat/completions", self.base_url.trim_end_matches('/'))
    }

    /// `/models` tam adresini üretir (model listesi çekmek için).
    pub fn models_url(&self) -> String {
        format!("{}/models", self.base_url.trim_end_matches('/'))
    }

    /// Kimliğe göre model arar.
    pub fn model(&self, model_id: &str) -> Option<&'static Model> {
        self.models.iter().find(|m| m.id == model_id)
    }

    /// Yalnız ücretsiz modelleri döner.
    pub fn free_models(&self) -> impl Iterator<Item = &'static Model> {
        self.models.iter().filter(|m| m.free)
    }

    /// En geniş bağlamlı modeli döner.
    pub fn widest_context(&self) -> Option<&'static Model> {
        self.models.iter().max_by_key(|m| m.context)
    }
}
