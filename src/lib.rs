//! # elytra-providers
//!
//! Elytra'nın sağlayıcı ve model kataloğu.
//!
//! ## Tasarım kararları
//! - **Yalnız bulut sağlayıcıları.** Yerel model (Ollama vb.) bilinçli olarak
//!   dışarıda — proje kuralı.
//! - **Uydurma model kimliği yok.** Kimlikler doğrulanmış kaynaklardan gelir
//!   (bkz. [`catalog`] modül başlığı).
//! - Katalog **statiktir** (derleme zamanı sabiti); istemci katmanı her
//!   sağlayıcının `/models` ucundan canlı listeyi çekip bununla birleştirebilir.
//!
//! ## Örnek
//! ```
//! use elytra_providers as ep;
//!
//! let p = ep::provider("groq").expect("groq katalogda yok");
//! println!("{} → {}", p.name, p.chat_url());
//!
//! for m in p.free_models() {
//!     println!("  {} ({} token)", m.name, m.context_label());
//! }
//! ```

pub mod catalog;
pub mod model;
pub mod provider;

pub use catalog::PROVIDERS;
pub use model::Model;
pub use provider::{ApiKind, Provider};

/// Tüm sağlayıcılar.
pub fn providers() -> &'static [Provider] {
    PROVIDERS
}

/// Kimliğe göre sağlayıcı arar (`"groq"`, `"openrouter"`, ...).
pub fn provider(id: &str) -> Option<&'static Provider> {
    PROVIDERS.iter().find(|p| p.id == id)
}

/// Ücretsiz katmanı olan sağlayıcılar.
pub fn free_tier_providers() -> impl Iterator<Item = &'static Provider> {
    PROVIDERS.iter().filter(|p| p.is_free_tier)
}

/// Tüm katalogdaki (sağlayıcı, model) çiftleri.
pub fn all_models() -> impl Iterator<Item = (&'static Provider, &'static Model)> {
    PROVIDERS
        .iter()
        .flat_map(|p| p.models.iter().map(move |m| (p, m)))
}

/// Katalogdaki ücretsiz (sağlayıcı, model) çiftleri.
pub fn free_models() -> impl Iterator<Item = (&'static Provider, &'static Model)> {
    all_models().filter(|(_, m)| m.free)
}

/// Sağlayıcı kimliği ile model kimliğinden çifti bulur.
pub fn resolve(provider_id: &str, model_id: &str) -> Option<(&'static Provider, &'static Model)> {
    let p = provider(provider_id)?;
    p.model(model_id).map(|m| (p, m))
}

/// Sağlayıcıya özel ortam değişkeninden anahtarı okur.
///
/// Anahtar yoksa `None` döner — çağıran taraf kullanıcıya "anahtar gir" der.
pub fn api_key(provider_id: &str) -> Option<String> {
    let p = provider(provider_id)?;
    std::env::var(p.env_key).ok().filter(|k| !k.trim().is_empty())
}

/// Katalog özeti: (sağlayıcı sayısı, toplam model, ücretsiz model).
pub fn summary() -> (usize, usize, usize) {
    let total: usize = PROVIDERS.iter().map(|p| p.models.len()).sum();
    let free = free_models().count();
    (PROVIDERS.len(), total, free)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn katalog_bos_degil() {
        let (p, m, f) = summary();
        assert!(p >= 10, "sağlayıcı sayısı beklenenden az: {p}");
        assert!(m >= 30, "model sayısı beklenenden az: {m}");
        assert!(f >= 20, "ücretsiz model sayısı beklenenden az: {f}");
    }

    #[test]
    fn yerel_saglayici_yok() {
        // Proje kuralı: Ollama / yerel model sağlayıcısı BULUNMAMALI.
        for p in providers() {
            let hay = format!("{} {} {}", p.id, p.name, p.base_url).to_lowercase();
            assert!(
                !hay.contains("ollama") && !hay.contains("local://") && !hay.contains("localhost"),
                "yerel sağlayıcı bulundu: {} ({})",
                p.id,
                p.base_url
            );
        }
    }

    #[test]
    fn kimlikler_tekil() {
        let mut ids: Vec<&str> = providers().iter().map(|p| p.id).collect();
        ids.sort_unstable();
        let n = ids.len();
        ids.dedup();
        assert_eq!(n, ids.len(), "yinelenen sağlayıcı kimliği var");
    }

    #[test]
    fn her_saglayicinin_modeli_var() {
        for p in providers() {
            assert!(!p.models.is_empty(), "{} sağlayıcısının modeli yok", p.id);
        }
    }

    #[test]
    fn urller_gecerli() {
        for p in providers() {
            assert!(
                p.base_url.starts_with("https://"),
                "{} base_url https değil: {}",
                p.id,
                p.base_url
            );
            assert!(
                !p.base_url.ends_with('/'),
                "{} base_url sonda / var: {}",
                p.id,
                p.base_url
            );
            assert!(
                !p.env_key.is_empty(),
                "{} ortam değişkeni boş",
                p.id
            );
            assert!(
                p.signup.starts_with("https://"),
                "{} signup adresi geçersiz",
                p.id
            );
        }
    }

    #[test]
    fn model_kimlikleri_dolu_ve_bagli_gecerli() {
        for (p, m) in all_models() {
            assert!(!m.id.trim().is_empty(), "{} içinde boş model kimliği", p.id);
            assert!(!m.name.trim().is_empty(), "{} / {} adı boş", p.id, m.id);
            assert!(
                m.context >= 1_000,
                "{} / {} bağlam çok küçük: {}",
                p.id,
                m.id,
                m.context
            );
        }
    }

    #[test]
    fn arama_ve_cozumleme_calisiyor() {
        let p = provider("groq").expect("groq yok");
        assert_eq!(p.chat_url(), "https://api.groq.com/openai/v1/chat/completions");
        assert_eq!(p.models_url(), "https://api.groq.com/openai/v1/models");

        let (pp, mm) = resolve("groq", "llama-3.3-70b-versatile").expect("model çözümlenemedi");
        assert_eq!(pp.id, "groq");
        assert_eq!(mm.context, 128_000);

        assert!(resolve("groq", "olmayan-model").is_none());
        assert!(provider("olmayan-saglayici").is_none());
    }

    #[test]
    fn baglam_etiketi_dogru() {
        let m = Model::new("x", "X", 1_048_576, true);
        assert_eq!(m.context_label(), "1.0M");
        let m2 = Model::new("y", "Y", 128_000, false);
        assert_eq!(m2.context_label(), "128K");
    }

    #[test]
    fn ucretsiz_saglayicilar_var() {
        let n = free_tier_providers().count();
        assert!(n >= 5, "ücretsiz katmanlı sağlayıcı sayısı az: {n}");
    }
}
