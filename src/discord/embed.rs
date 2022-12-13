#![allow(dead_code)]

use twilight_model::{
    channel::message::{
        embed::{
            EmbedAuthor as APIEmbedAuthor, EmbedField as APIEmbedField,
            EmbedFooter as APIEmbedFooter, EmbedImage as APIEmbedImage,
            EmbedThumbnail as APIEmbedThumbnail,
        },
        Embed as APIEmbed,
    },
    util::Timestamp,
};

use crate::util::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedAuthor {
    pub name: String,
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Embed {
    author: Option<EmbedAuthor>,
    color: Option<Color>,
    title: Option<String>,
    image: Option<String>,
    thumbnail: Option<String>,
    description: Option<String>,
    fields: Vec<EmbedField>,
    timestamp: Option<Timestamp>,
    footer: Option<EmbedFooter>,
}

impl Embed {
    fn build(self) -> APIEmbed {
        APIEmbed {
            author: self.author.map(|a| APIEmbedAuthor {
                name: a.name,
                icon_url: a.icon_url,
                proxy_icon_url: None,
                url: None,
            }),
            color: self.color.map(|c| c.to_u32()),
            title: self.title,
            description: self.description,
            fields: self
                .fields
                .iter()
                .map(|f| APIEmbedField {
                    inline: f.inline,
                    name: f.name.clone(),
                    value: f.value.clone(),
                })
                .collect(),
            footer: self.footer.map(|f| APIEmbedFooter {
                text: f.text,
                icon_url: f.icon_url,
                proxy_icon_url: None,
            }),
            image: self.image.map(|img| APIEmbedImage {
                url: img,
                height: None,
                width: None,
                proxy_url: None,
            }),
            kind: "Embed".to_string(),
            provider: None,
            thumbnail: self.thumbnail.map(|t| APIEmbedThumbnail {
                url: t,
                width: None,
                height: None,
                proxy_url: None,
            }),
            timestamp: self.timestamp,
            url: None,
            video: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EmbedBuilder {
    pub embed: Embed,
}

impl EmbedBuilder {
    pub fn new() -> EmbedBuilder {
        Self {
            embed: Embed::default(),
        }
    }

    pub fn build(self) -> APIEmbed {
        self.embed.build()
    }

    pub fn set_author(&mut self, author: EmbedAuthor) -> Self {
        self.embed.author = Some(author);
        self.to_owned()
    }

    pub fn set_color(&mut self, color: Color) -> Self {
        self.embed.color = Some(color);
        self.to_owned()
    }

    pub fn set_title<S: ToString>(&mut self, title: S) -> Self {
        self.embed.title = Some(title.to_string());
        self.to_owned()
    }

    pub fn set_image<S: ToString>(&mut self, image: S) -> Self {
        self.embed.image = Some(image.to_string());
        self.to_owned()
    }

    pub fn set_thumbnail<S: ToString>(&mut self, thumbnail: S) -> Self {
        self.embed.thumbnail = Some(thumbnail.to_string());
        self.to_owned()
    }

    pub fn set_description<S: ToString>(&mut self, description: S) -> Self {
        self.embed.description = Some(description.to_string());
        self.to_owned()
    }

    pub fn set_timestamp(&mut self, timestamp: Timestamp) -> Self {
        self.embed.timestamp = Some(timestamp);
        self.to_owned()
    }

    pub fn set_current_timestamp(&mut self) -> Self {
        let timestamp = Timestamp::parse(chrono::Utc::now().to_rfc3339().as_str());

        if let Ok(timestamp) = timestamp {
            return self.set_timestamp(timestamp);
        }

        self.to_owned()
    }

    pub fn set_footer(&mut self, footer: EmbedFooter) -> Self {
        self.embed.footer = Some(footer);
        self.to_owned()
    }

    pub fn add_field(&mut self, field: EmbedField) -> Self {
        self.embed.fields.push(field);
        self.to_owned()
    }

    pub fn add_fields(&mut self, fields: &mut Vec<EmbedField>) -> Self {
        self.embed.fields.append(fields);
        self.to_owned()
    }
}
