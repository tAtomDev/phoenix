use twilight_model::user::User;

pub trait UserExtension {
    fn avatar_url(&self) -> String;
}

impl UserExtension for User {
    fn avatar_url(&self) -> String {
        let Some(avatar) = self.avatar else {
            return "https://external-preview.redd.it/fauTrGFvbnTjWM6A6AC8sGqohLQxKHQTfZjhtPbWY7g.jpg?auto=webp&s=5d8e36356dead73ec2e624e41659d411b5fbca53".into();
        };

        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            self.id,
            avatar
        )
    }
}
