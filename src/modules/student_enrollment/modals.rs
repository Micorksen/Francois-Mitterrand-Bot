use poise::serenity_prelude as serenity;

use super::structs::{Enrollment, Level};
use crate::Data;

/// Première étape de l'inscription en tant qu'élève.
#[derive(poise::Modal)]
#[name = "Mon inscription en tant qu'élève"]
pub struct FirstStep {
    #[name = "Prénom"]
    #[placeholder = "François"]
    pub first_name: serenity::small_fixed_array::FixedString<u16>,

    #[name = "Nom de famille"]
    #[placeholder = "MITTERRAND"]
    pub last_name: serenity::small_fixed_array::FixedString<u16>,

    #[name = "Sexe"]
    #[radio_group("Masculin", "Féminin")]
    pub sex: serenity::small_fixed_array::FixedString,

    #[name = "Niveau"]
    #[string_select("Seconde", "Première", "Terminale")]
    pub level: serenity::small_fixed_array::FixedArray<String>,

    #[name = "Régime"]
    #[radio_group("Interne", "Externe")]
    pub regime: serenity::small_fixed_array::FixedString,
}

/// Dernière étape de l'inscription en tant qu'élève.
pub fn final_step<'a>(data: &'a Data, enrollment: &Enrollment) -> serenity::CreateModal<'a> {
    let mut components = vec![
        serenity::CreateModalComponent::Label(serenity::CreateLabel::select_menu(
            "Première langue vivante étrangère",
            serenity::CreateSelectMenu::new(
                "lv1",
                serenity::CreateSelectMenuKind::String {
                    options: data
                        .config
                        .student_enrollment
                        .lv1
                        .iter()
                        .map(|(k, v)| serenity::CreateSelectMenuOption::new(v, k))
                        .collect(),
                },
            ),
        )),
        serenity::CreateModalComponent::Label(serenity::CreateLabel::select_menu(
            "Deuxième langue vivante étrangère",
            serenity::CreateSelectMenu::new(
                "lv2",
                serenity::CreateSelectMenuKind::String {
                    options: data
                        .config
                        .student_enrollment
                        .lv2
                        .iter()
                        .map(|(k, v)| serenity::CreateSelectMenuOption::new(v, k))
                        .collect(),
                },
            ),
        )),
    ];

    // Les élèves de première peuvent choisir deux matières de spécialité parmi deux groupes.
    if enrollment.level == Level::Premiere {
        components.push(serenity::CreateModalComponent::Label(
            serenity::CreateLabel::select_menu(
                "Première matière de spécialité",
                serenity::CreateSelectMenu::new(
                    "first_specialized_subject",
                    serenity::CreateSelectMenuKind::String {
                        options: data.config.student_enrollment.specialized_subjects[0]
                            .iter()
                            .map(|(k, v)| serenity::CreateSelectMenuOption::new(v, k))
                            .collect(),
                    },
                ),
            ),
        ));

        components.push(serenity::CreateModalComponent::Label(
            serenity::CreateLabel::select_menu(
                "Deuxième matière de spécialité",
                serenity::CreateSelectMenu::new(
                    "second_specialized_subject",
                    serenity::CreateSelectMenuKind::String {
                        options: data.config.student_enrollment.specialized_subjects[1]
                            .iter()
                            .map(|(k, v)| serenity::CreateSelectMenuOption::new(v, k))
                            .collect(),
                    },
                ),
            ),
        ));
    }

    // Les élèves de terminale peuvent choisir une seule matière de spécialité.
    // Ils peuvent également choisir une option parmi celles proposées.
    if enrollment.level == Level::Terminale {
        components.push(serenity::CreateModalComponent::Label(
            serenity::CreateLabel::select_menu(
                "Matière de spécialité",
                serenity::CreateSelectMenu::new(
                    "first_specialized_subject",
                    serenity::CreateSelectMenuKind::String {
                        options: data
                            .config
                            .student_enrollment
                            .specialized_subjects
                            .iter()
                            .flat_map(|slice| slice.iter())
                            .map(|(k, v)| serenity::CreateSelectMenuOption::new(v, k))
                            .collect(),
                    },
                ),
            ),
        ));

        components.push(serenity::CreateModalComponent::Label(
            serenity::CreateLabel::select_menu(
                "Option",
                serenity::CreateSelectMenu::new(
                    "option",
                    serenity::CreateSelectMenuKind::String {
                        options: data
                            .config
                            .student_enrollment
                            .options
                            .iter()
                            .map(|(k, v)| serenity::CreateSelectMenuOption::new(v, k))
                            .collect(),
                    },
                )
                .required(false),
            ),
        ));
    }

    serenity::CreateModal::new(
        "student_enrollment:final_step",
        "Mon inscription en tant qu'élève",
    )
    .components(components)
}
