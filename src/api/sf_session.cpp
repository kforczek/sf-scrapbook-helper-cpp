#include "sf_session.h"

#include "assert.h"
#include "sf_api.hpp"
#include <iostream>

namespace
{

using ffi_char_t = int8_t;

const ffi_char_t* to_ffi_str(const std::string& str)
{
    return reinterpret_cast<const ffi_char_t*>(str.c_str());
}

std::string to_str(const ffi_char_t* ffiStr)
{
    return reinterpret_cast<const char*>(ffiStr);
}

} // namespace


namespace sf
{

Session::Session(const std::string& username, const std::string& password, const std::string& serverUrl)
{
    m_session = ffi::init_session(to_ffi_str(username), to_ffi_str(password), to_ffi_str(serverUrl));
    if (!m_session)
        throw std::runtime_error("Failed to create session");

    bool isLoginSuccessful = ffi::login(m_session);
    if (!isLoginSuccessful)
        throw std::runtime_error("Invalid login credentials");
}

Session::~Session()
{
    if (m_session)
        ffi::destr_session(m_session);
}

CommandResponse Session::update() { return ffi::exec_Update(m_session); }
CommandResponse Session::buyBeer() { return ffi::exec_BuyBeer(m_session); }
CommandResponse Session::cancelQuest() { return ffi::exec_CancelQuest(m_session); }
CommandResponse Session::finishWork() { return ffi::exec_FinishWork(m_session); }
CommandResponse Session::checkArena() { return ffi::exec_CheckArena(m_session); }
CommandResponse Session::collectCalendar() { return ffi::exec_CollectCalendar(m_session); }
CommandResponse Session::toiletFlush() { return ffi::exec_ToiletFlush(m_session); }
CommandResponse Session::toiletOpen() { return ffi::exec_ToiletOpen(m_session); }
CommandResponse Session::cancelWork() { return ffi::exec_CancelWork(m_session); }
CommandResponse Session::guildLoadMushrooms() { return ffi::exec_GuildLoadMushrooms(m_session); }
CommandResponse Session::guildJoinAttack() { return ffi::exec_GuildJoinAttack(m_session); }
CommandResponse Session::guildJoinDefense() { return ffi::exec_GuildJoinDefense(m_session); }
CommandResponse Session::guildRaid() { return ffi::exec_GuildRaid(m_session); }
CommandResponse Session::guildPortalBattle() { return ffi::exec_GuildPortalBattle(m_session); }
CommandResponse Session::guildGetFightableTargets() { return ffi::exec_GuildGetFightableTargets(m_session); }
CommandResponse Session::viewScrapbook() { return ffi::exec_ViewScrapbook(m_session); }
CommandResponse Session::fightPortal() { return ffi::exec_FightPortal(m_session); }
CommandResponse Session::swapManequin() { return ffi::exec_SwapManequin(m_session); }
CommandResponse Session::idleSacrifice() { return ffi::exec_IdleSacrifice(m_session); }
CommandResponse Session::hellevatorEnter() { return ffi::exec_HellevatorEnter(m_session); }
CommandResponse Session::hellevatorViewGuildRanking() { return ffi::exec_HellevatorViewGuildRanking(m_session); }
CommandResponse Session::hellevatorRefreshShop() { return ffi::exec_HellevatorRefreshShop(m_session); }
CommandResponse Session::hellevatorClaimDaily() { return ffi::exec_HellevatorClaimDaily(m_session); }
CommandResponse Session::hellevatorClaimDailyYesterday() { return ffi::exec_HellevatorClaimDailyYesterday(m_session); }
CommandResponse Session::hellevatorClaimFinal() { return ffi::exec_HellevatorClaimFinal(m_session); }
CommandResponse Session::hellevatorPreviewRewards() { return ffi::exec_HellevatorPreviewRewards(m_session); }
CommandResponse Session::buyGoldFrame() { return ffi::exec_BuyGoldFrame(m_session); }
CommandResponse Session::fortressGemStoneSearch() { return ffi::exec_FortressGemStoneSearch(m_session); }
CommandResponse Session::fortressGemStoneSearchCancel() { return ffi::exec_FortressGemStoneSearchCancel(m_session); }
CommandResponse Session::fortressUpgradeHallOfKnights() { return ffi::exec_FortressUpgradeHallOfKnights(m_session); }

CommandResponse Session::hallOfFamePage(size_t page) { return ffi::exec_HallOfFamePage(m_session, page); }
CommandResponse Session::hallOfFameFortressPage(size_t page) { return ffi::exec_HallOfFameFortressPage(m_session, page); }
CommandResponse Session::viewPlayer(const std::string& ident) { return ffi::exec_ViewPlayer(m_session, to_ffi_str(ident)); }
CommandResponse Session::startQuest(size_t questPos, bool overwriteInv) { return ffi::exec_StartQuest(m_session, questPos, overwriteInv); }
CommandResponse Session::removePotion(size_t pos) { return ffi::exec_RemovePotion(m_session, pos); }
CommandResponse Session::fight(const std::string& name, bool useMushroom) { return ffi::exec_Fight(m_session, to_ffi_str(name), useMushroom); }
CommandResponse Session::guildAttack(const std::string& guild) { return ffi::exec_GuildAttack(m_session, to_ffi_str(guild)); }
CommandResponse Session::finishQuest(bool skip) { return ffi::exec_FinishQuest(m_session, skip); }
CommandResponse Session::checkNameAvailable(const std::string& name) { return ffi::exec_CheckNameAvailable(m_session, to_ffi_str(name)); }
CommandResponse Session::viewGuild(const std::string& guildIdent) { return ffi::exec_ViewGuild(m_session, to_ffi_str(guildIdent)); }
CommandResponse Session::guildFound(const std::string& name) { return ffi::exec_GuildFound(m_session, to_ffi_str(name)); }
CommandResponse Session::guildInvitePlayer(const std::string& name) { return ffi::exec_GuildInvitePlayer(m_session, to_ffi_str(name)); }
CommandResponse Session::guildKickPlayer(const std::string& name) { return ffi::exec_GuildKickPlayer(m_session, to_ffi_str(name)); }
CommandResponse Session::guildSetLeader(const std::string& name) { return ffi::exec_GuildSetLeader(m_session, to_ffi_str(name)); }
CommandResponse Session::guildToggleOfficer(const std::string& name) { return ffi::exec_GuildToggleOfficer(m_session, to_ffi_str(name)); }
CommandResponse Session::messageOpen(int pos) { return ffi::exec_MessageOpen(m_session, pos); }
CommandResponse Session::messageDelete(int pos) { return ffi::exec_MessageDelete(m_session, pos); }
CommandResponse Session::viewPet(unsigned short petId) { return ffi::exec_ViewPet(m_session, petId); }
CommandResponse Session::gambleSilver(unsigned long amount) { return ffi::exec_GambleSilver(m_session, amount); }
CommandResponse Session::gambleMushrooms(unsigned long amount) { return ffi::exec_GambleMushrooms(m_session, amount); }
CommandResponse Session::sendMessage(const std::string& to, const std::string& msg) { return ffi::exec_SendMessage(m_session, to_ffi_str(to), to_ffi_str(msg)); }
CommandResponse Session::whisper(const std::string& playerName, const std::string& message) { return ffi::exec_Whisper(m_session, to_ffi_str(playerName), to_ffi_str(message)); }
CommandResponse Session::setLanguage(const std::string& language) { return ffi::exec_SetLanguage(m_session, to_ffi_str(language)); }
CommandResponse Session::fortressNewEnemy(bool useMushroom) { return ffi::exec_FortressNewEnemy(m_session, useMushroom); }
CommandResponse Session::fortressSetCAEnemy(unsigned int msgId) { return ffi::exec_FortressSetCAEnemy(m_session, msgId); }
CommandResponse Session::guildPetBattle(bool useMushroom) { return ffi::exec_GuildPetBattle(m_session, useMushroom); }
CommandResponse Session::hallOfFameGroupPage(unsigned int page) { return ffi::exec_HallOfFameGroupPage(m_session, page); }
CommandResponse Session::hallOfFameUnderworldPage(unsigned int page) { return ffi::exec_HallOfFameUnderworldPage(m_session, page); }
CommandResponse Session::hallOfFamePetsPage(unsigned int page) { return ffi::exec_HallOfFamePetsPage(m_session, page); }
CommandResponse Session::blockGuildInvites(bool blockInvites) { return ffi::exec_BlockGuildInvites(m_session, blockInvites); }
CommandResponse Session::showTips(bool showTips) { return ffi::exec_ShowTips(m_session, showTips); }
CommandResponse Session::unlockFeature(long mainIdent, long subIdent) { return ffi::exec_UnlockFeature(m_session, mainIdent, subIdent); }
CommandResponse Session::startWork(unsigned short hours) {
    static uint8_t MAX_VAL = std::numeric_limits<uint8_t>::max();
    if (hours > MAX_VAL)
        throw std::runtime_error("Exceeded maximum hours (" + std::to_string(MAX_VAL) + ")");

    return ffi::exec_StartWork(m_session, (uint8_t) hours);
}


CommandResponse Session::fortressBuild(FortressBuildingType building) { return ffi::exec_FortressBuild(m_session, static_cast<uint8_t>(building)); }
CommandResponse Session::fortressBuildCancel(FortressBuildingType building) { return ffi::exec_FortressBuildCancel(m_session, static_cast<uint8_t>(building)); }
CommandResponse Session::fortressBuildFinish(FortressBuildingType building, unsigned int mushrooms) { return ffi::exec_FortressBuildFinish(m_session, static_cast<uint8_t>(building), mushrooms); }
CommandResponse Session::fortressBuildUnit(FortressUnitType unit, unsigned int count) { return ffi::exec_FortressBuildUnit(m_session, static_cast<uint8_t>(unit), count); }
CommandResponse Session::fortressGather(FortressResourceType resource) { return ffi::exec_FortressGather(m_session, static_cast<uint8_t>(resource)); }
CommandResponse Session::underworldCollect(UnderworldResourceType resource) { return ffi::exec_UnderworldCollect(m_session, static_cast<uint8_t>(resource)); }
CommandResponse Session::underworldUnitUpgrade(UnderworldUnitType unit) { return ffi::exec_UnderworldUnitUpgrade(m_session, static_cast<uint8_t>(unit)); }
CommandResponse Session::underworldUpgradeCancel(UnderworldUnitType unit) { return ffi::exec_UnderworldUpgradeCancel(m_session, static_cast<uint8_t>(unit)); }
CommandResponse Session::underworldUpgradeFinish(UnderworldBuildingType building, unsigned int mushrooms) { return ffi::exec_UnderworldUpgradeFinish(m_session, static_cast<uint8_t>(building), mushrooms); }
CommandResponse Session::spinWheelOfFortune(FortunePaymentType payment) { return ffi::exec_SpinWheelOfFortune(m_session, static_cast<uint8_t>(payment)); }
CommandResponse Session::buyMount(MountType mount) { return ffi::exec_BuyMount(m_session, static_cast<uint8_t>(mount)); }
CommandResponse Session::increaseAttribute(AttributeType attribute, unsigned int increaseTo) { return ffi::exec_IncreaseAttribute(m_session, static_cast<uint8_t>(attribute), increaseTo); }
CommandResponse Session::guildIncreaseSkill(GuildSkillType skill, unsigned short current) { return ffi::exec_GuildIncreaseSkill(m_session, static_cast<uint8_t>(skill), current); }
CommandResponse Session::toiletDrop(PlayerItemPlaceType inventory, size_t pos) { return ffi::exec_ToiletDrop(m_session, static_cast<uint8_t>(inventory), pos); }


CommandResponse::CommandResponse(ffi::Response* response)
    : m_response(response) { }

CommandResponse::~CommandResponse()
{
    if (m_response)
        ffi::destr_response(m_response);
}

bool CommandResponse::isValid() const
{
    return m_response != nullptr;
}

std::unordered_set<std::string> CommandResponse::getKeys() const
{
    size_t keysCnt;
    const ffi_char_t** rawKeys = ffi::response_get_keys(m_response, &keysCnt);

    std::unordered_set<std::string> set;
    for (size_t i = 0; i < keysCnt; ++i)
        set.insert(to_str(rawKeys[i]));

    ffi::destr_response_keys(rawKeys, keysCnt);
    return set;
}

std::string CommandResponse::getValue(const std::string& key) const
{
    ffi_char_t* rawVal = ffi::response_get_value(m_response, to_ffi_str(key));
    
    std::string val = to_str(rawVal);
    ffi::destr_response_value(rawVal);

    return val;
}

} // namespace sf