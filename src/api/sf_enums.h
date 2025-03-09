#pragma once

namespace sf
{

enum class FortressBuildingType
{
    Fortress,
    LaborersQuarters,
    WoodcuttersHut,
    Quarry,
    GemMine,
    Academy,
    ArcheryGuild,
    Barracks,
    MagesTower,
    Treasury,
    Smithy,
    Wall
}; // FortressBuildingType


enum class FortressUnitType
{
    Soldier,
    Magician,
    Archer
}; // FortressUnitType


enum class FortressResourceType
{
    Wood,
    Stone,
    Experience
}; // FortressResourceType


enum class UnderworldResourceType
{
    Souls,
    Silver,
    ThirstForAdventure
}; // UnderworldResourceType


enum class UnderworldUnitType
{
    Goblin,
    Troll,
    Keeper
}; // UnderworldUnitType


enum class UnderworldBuildingType
{
    HeartOfDarkness,
    Gate,
    GoldPit,
    SoulExtractor,
    GoblinPit,
    TortureChamber,
    GladiatorTrainer,
    TrollBlock,
    Adventuromatic,
    Keeper
}; // UnderworldBuildingType


enum class FortunePaymentType
{
    LuckyCoins,
    Mushrooms,
    FreeTurn
}; // FortunePayment


enum class MountType
{
    Cow = 1,
    Horse,
    Tiger,
    Dragon
}; // MountType


enum class AttributeType
{
    Strength = 1,
    Dexterity,
    Intelligence,
    Constitution,
    Luck
}; // AttributeType


// Something the player can upgrade in the guild
enum class GuildSkillType
{
    Treasure,
    Instructor,
    Pet
}; // GuildSkillType


// All the parts of `ItemPlace`, that are owned by the player
enum class PlayerItemPlaceType
{
    Equipment = 1,
    MainInventory = 2,
    ExtendedInventory = 5
}; // PlayerItemPlace

} // namespace sf