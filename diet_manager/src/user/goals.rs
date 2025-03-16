use crate::models::{ UserProfile, Gender, ActivityLevel, CalorieCalculationMethod };

pub trait CalorieCalculator {
    fn calculate_bmr(&self, profile: &UserProfile) -> f32;
    fn calculate_tdee(&self, profile: &UserProfile) -> f32 {
        let bmr = self.calculate_bmr(profile);
        let activity_multiplier = match profile.activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::Light => 1.375, // Changed from LightlyActive
            ActivityLevel::Moderate => 1.55, // Changed from ModeratelyActive
            ActivityLevel::VeryActive => 1.725,
            ActivityLevel::ExtraActive => 1.9, // Changed from ExtremelyActive
        };
        bmr * activity_multiplier
    }
}

pub struct HarrisBenedictCalculator;

impl CalorieCalculator for HarrisBenedictCalculator {
    fn calculate_bmr(&self, profile: &UserProfile) -> f32 {
        match profile.gender {
            Gender::Male =>
                88.362 +
                    13.397 * profile.weight_kg +
                    4.799 * profile.height_cm -
                    5.677 * (profile.age as f32),
            Gender::Female =>
                447.593 +
                    9.247 * profile.weight_kg +
                    3.098 * profile.height_cm -
                    4.33 * (profile.age as f32),

        }
    }
}

pub struct MifflinStJeorCalculator;

impl CalorieCalculator for MifflinStJeorCalculator {
    fn calculate_bmr(&self, profile: &UserProfile) -> f32 {
        let gender_factor = match profile.gender {
            Gender::Male => 5.0,
            Gender::Female => -161.0,
           // Average of male and female factors
        };

        10.0 * profile.weight_kg +
            6.25 * profile.height_cm -
            5.0 * (profile.age as f32) +
            gender_factor
    }
}

pub fn get_calculator(method: CalorieCalculationMethod) -> Box<dyn CalorieCalculator> {
    match method {
        CalorieCalculationMethod::HarrisBenedict => Box::new(HarrisBenedictCalculator),
        CalorieCalculationMethod::MifflinStJeor => Box::new(MifflinStJeorCalculator),
    }
}

pub fn calculate_daily_goal(
    profile: &UserProfile,
    weight_kg: f32,
    activity_level: ActivityLevel
) -> f32 {
    let temp_profile = UserProfile {
        weight_kg,
        activity_level,
        ..profile.clone()
    };

    let calculator = get_calculator(profile.calorie_method);
    calculator.calculate_tdee(&temp_profile)
}
