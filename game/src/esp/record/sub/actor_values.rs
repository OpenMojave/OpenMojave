use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, TryFromPrimitive, PartialEq, Eq)]
#[repr(i8)]
pub enum ActorValues {
    None = -1,
    Aggression = 0,
    Confidence = 1,
    Energy = 2,
    Responsibility = 3,
    Mood = 4,
    Strength = 5,
    Perception = 6,
    Endurance = 7,
    Charisma = 8,
    Intelligence = 9,
    Agility = 10,
    Luck = 11,
    ActionPoints = 12,
    CarryWeight = 13,
    CriticalChance = 14,
    HealRate = 15,
    Health = 16,
    MeleeDamage = 17,
    DamageResistance = 18,
    PoisonResistance = 19,
    RadResistance = 20,
    SpeedMultiplier = 21,
    Fatigue = 22,
    Karma = 23,
    Xp = 24,
    PerceptionCondition = 25,
    EduranceCondition = 26,
    LeftAttackCondition = 27,
    RightAttackCondition = 28,
    LeftMobilityCondition = 29,
    RightMobilityCondition = 30,
    BrainCondition = 31,
    Barter = 32,
    BigGunsUnused = 33,
    EnergyWeapons = 34,
    Explosives = 35,
    Lockpick = 36,
    Medicine = 37,
    MeleeWeapons = 38,
    Repair = 39,
    Science = 40,
    Guns = 41,
    Sneak = 42,
    Speech = 43,
    Survival = 44,
    Unarmed = 45,
    InventoryWeight = 46,
    Paralysis = 47,
    Invisibility = 48,
    Chameleon = 49,
    NightEye = 50,
    Turbo = 51,
    FireResistance = 52,
    WaterBreathing = 53,
    RadLevel = 54,
    BloodyMess = 55,
    UnarmedDamage = 56,
    Assistance = 57,
    ElectricResistance = 58,
    FrostResistance = 59,
    EnergyResistance = 60,
    EMPResistance = 61,
    U1 = 62,
    U2 = 63,
    U4 = 64,
    U5 = 65,
    U6 = 66,
    U7 = 67,
    U8 = 68,
    U9 = 69,
    U10 = 70,
    U11 = 71,
    IgnoreCrippledLimbs = 72,
    Dehydration = 73,
    Hunger = 74,
    SleepDeprivation = 75,
    Damage = 76,
}
