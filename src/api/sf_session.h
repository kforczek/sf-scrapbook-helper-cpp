#pragma once
#include <string>
#include <unordered_set>
#include "sf_enums.h"


namespace ffi
{
    class Session;
    class Response;
}

namespace sf
{

class CommandResponse;


class Session
{
public:
    Session(const std::string& username, const std::string& password, const std::string& serverUrl);
    ~Session();

    /* No-argument commands */
    CommandResponse update();
    CommandResponse buyBeer();
    CommandResponse cancelQuest();
    CommandResponse finishWork();
    CommandResponse checkArena();
    CommandResponse collectCalendar();
    CommandResponse toiletFlush();
    CommandResponse toiletOpen();
    CommandResponse cancelWork();
    CommandResponse guildLoadMushrooms();
    CommandResponse guildJoinAttack();
    CommandResponse guildJoinDefense();
    CommandResponse guildRaid();
    CommandResponse guildPortalBattle();
    CommandResponse guildGetFightableTargets();
    CommandResponse viewScrapbook();
    CommandResponse fightPortal();
    CommandResponse swapManequin();
    CommandResponse idleSacrifice();
    CommandResponse hellevatorEnter();
    CommandResponse hellevatorViewGuildRanking();
    CommandResponse hellevatorRefreshShop();
    CommandResponse hellevatorClaimDaily();
    CommandResponse hellevatorClaimDailyYesterday();
    CommandResponse hellevatorClaimFinal();
    CommandResponse hellevatorPreviewRewards();
    CommandResponse buyGoldFrame();
    CommandResponse fortressGemStoneSearch();
    CommandResponse fortressGemStoneSearchCancel();
    CommandResponse fortressUpgradeHallOfKnights();
    CommandResponse expeditionContinue();
    

    /* Commands with trivial arguments */
    CommandResponse hallOfFamePage(size_t page);
    CommandResponse hallOfFameFortressPage(size_t page);
    CommandResponse viewPlayer(const std::string& ident);
    CommandResponse startQuest(size_t questPos, bool overwriteInv);
    CommandResponse startWork(unsigned short hours);
    CommandResponse removePotion(size_t pos);
    CommandResponse fight(const std::string& name, bool useMushroom);
    CommandResponse guildAttack(const std::string& guild);
    CommandResponse finishQuest(bool skip);
    CommandResponse checkNameAvailable(const std::string& name);
    CommandResponse viewGuild(const std::string& guildIdent);
    CommandResponse guildFound(const std::string& name);
    CommandResponse guildInvitePlayer(const std::string& name);
    CommandResponse guildKickPlayer(const std::string& name);
    CommandResponse guildSetLeader(const std::string& name);
    CommandResponse guildToggleOfficer(const std::string& name);
    CommandResponse messageOpen(int pos);
    CommandResponse messageDelete(int pos);
    CommandResponse viewPet(unsigned short petId);
    CommandResponse gambleSilver(unsigned long amount);
    CommandResponse gambleMushrooms(unsigned long amount);
    CommandResponse sendMessage(const std::string& to, const std::string& msg);
    CommandResponse whisper(const std::string& playerName, const std::string& message);
    CommandResponse setLanguage(const std::string& language);
    CommandResponse fortressNewEnemy(bool useMushroom);
    CommandResponse fortressSetCAEnemy(unsigned int msgId);
    CommandResponse guildPetBattle(bool useMushroom);
    CommandResponse hallOfFameGroupPage(unsigned int page);
    CommandResponse hallOfFameUnderworldPage(unsigned int page);
    CommandResponse hallOfFamePetsPage(unsigned int page);
    CommandResponse blockGuildInvites(bool blockInvites);
    CommandResponse showTips(bool showTips);
    CommandResponse unlockFeature(long mainIdent, long subIdent);
    CommandResponse expeditionStart(size_t pos);
    CommandResponse expeditionPickEncounter(size_t pos);
    CommandResponse expeditionPickReward(size_t pos);
    CommandResponse hallOfFameHellevatorPage(size_t page);
    CommandResponse claimablePreview(long int msgId);
    CommandResponse claimableClaim(long int msgId);
    CommandResponse hellevatorJoinHellAttack(bool useMushroom, size_t plain);
    CommandResponse hellevatorFight(bool useMushroom);
    CommandResponse fortressGemStoneSearchFinish(unsigned int mushrooms);
    CommandResponse fortressAttack(unsigned int soldiers);
    CommandResponse fortressGatherSecretStorage(size_t stone, size_t wood);


    /* Commands with custom enum types */
    CommandResponse fortressBuild(FortressBuildingType building);
    CommandResponse fortressBuildCancel(FortressBuildingType building);
    CommandResponse fortressBuildFinish(FortressBuildingType building, unsigned int mushrooms);
    CommandResponse fortressBuildUnit(FortressUnitType unit, unsigned int count);
    CommandResponse fortressGather(FortressResourceType resource);
    CommandResponse underworldCollect(UnderworldResourceType resource);
    CommandResponse underworldUnitUpgrade(UnderworldUnitType unit);
    CommandResponse underworldUpgradeCancel(UnderworldUnitType unit);
    CommandResponse underworldUpgradeFinish(UnderworldBuildingType building, unsigned int mushrooms);
    CommandResponse spinWheelOfFortune(FortunePaymentType payment);
    CommandResponse buyMount(MountType mount);
    CommandResponse increaseAttribute(AttributeType attribute, unsigned int increaseTo);
    CommandResponse guildIncreaseSkill(GuildSkillType skill, unsigned short current);
    CommandResponse toiletDrop(PlayerItemPlaceType inventory, size_t pos);
    CommandResponse expeditionSkipWait(TimeSkipType type);
    CommandResponse hellevatorBuy(size_t position, HellevatorTreatType treat, unsigned int price, bool useMushroom);


private:
    ffi::Session* m_session;
};


class CommandResponse
{
public:
    CommandResponse(ffi::Response* response);
    ~CommandResponse();

    bool isValid() const;

    std::unordered_set<std::string> getKeys() const;
    std::string getValue(const std::string& key) const;

private:
    ffi::Response* m_response;
};

} // namespace sf
