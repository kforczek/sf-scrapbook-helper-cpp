#pragma once

namespace sf
{

// The type of building, that can be built in the fortress
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


// The type of a unit usable in the fortress
enum class FortressUnitType
{
    Soldier,
    Magician,
    Archer
}; // FortressUnitType


// The type of resource available in the fortress
enum class FortressResourceType
{
    Wood,
    Stone,
    Experience
}; // FortressResourceType


// The type of a producible resource in the underworld
enum class UnderworldResourceType
{
    Souls,
    Silver,
    ThirstForAdventure
}; // UnderworldResourceType


// The type of unit in the underworld
enum class UnderworldUnitType
{
    Goblin,
    Troll,
    Keeper
}; // UnderworldUnitType


// The type of building in the underworld
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


// A type of attribute
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