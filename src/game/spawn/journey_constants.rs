use super::weather::{DayWeather, Heat, Moisture, Wind};

const MONSOON: DayWeather = DayWeather {
    wind: Wind::GaleForce,
    heat: Heat::Freezing,
    moisture: Moisture::Humid,
};
const BLISTERING_DEATH: DayWeather = DayWeather {
    wind: Wind::None,
    heat: Heat::Blistering,
    moisture: Moisture::Dry,
};
const FREEZING_HELL: DayWeather = DayWeather {
    wind: Wind::None,
    heat: Heat::Freezing,
    moisture: Moisture::Dry,
};
const FREEZING_DEATH: DayWeather = DayWeather {
    wind: Wind::GaleForce,
    heat: Heat::Freezing,
    moisture: Moisture::Dry,
};
const SCORCHING_DESERT: DayWeather = DayWeather {
    wind: Wind::Low,
    heat: Heat::Blistering,
    moisture: Moisture::Dry,
};
const HUMID_SWAMP: DayWeather = DayWeather {
    wind: Wind::Low,
    heat: Heat::Blistering,
    moisture: Moisture::Humid,
};
const MILD_SUMMER: DayWeather = DayWeather {
    wind: Wind::Medium,
    heat: Heat::Warm,
    moisture: Moisture::Dry,
};
const COLD_FRONT: DayWeather = DayWeather {
    wind: Wind::High,
    heat: Heat::Chilly,
    moisture: Moisture::Dry,
};
const TROPICAL_STORM: DayWeather = DayWeather {
    wind: Wind::GaleForce,
    heat: Heat::Warm,
    moisture: Moisture::Humid,
};
const AUTUMN_BREEZE: DayWeather = DayWeather {
    wind: Wind::Medium,
    heat: Heat::Chilly,
    moisture: Moisture::Dry,
};
const SPRING_SHOWER: DayWeather = DayWeather {
    wind: Wind::Low,
    heat: Heat::Warm,
    moisture: Moisture::Humid,
};
const COOL_DRIZZLE: DayWeather = DayWeather {
    wind: Wind::Medium,
    heat: Heat::Chilly,
    moisture: Moisture::Humid,
};
