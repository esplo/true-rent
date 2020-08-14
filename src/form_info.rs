use crate::constants::{InputId, UnitId};
use crate::form_items;

pub fn create_form() -> form_items::Forms {
    form_items::Forms::new(
        vec![
            form_items::RentItem::new(
                "賃料",
                vec![UnitId::MonthlyYen],
                &InputId::RentUnit.to_string(),
                "毎月請求される家賃の基本。これだけで済むシンプルなルールなら、こんなツールは要らなかった。",
                &InputId::Rent.to_string(),
                form_items::InputValidation {
                    default_value: Some(50000),
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "共益費・管理費",
                vec![UnitId::MonthlyYen],
                &InputId::ManagementFeeUnit.to_string(),
                "毎月請求される基本的な料金その2。家賃と並んで市民権を得ているので、大々的に表示される。家賃○か月分、という費用にはこれが含まれないことが多いのでややこしい。",
                &InputId::ManagementFee.to_string(),
                form_items::InputValidation {
                    default_value: Some(2000),
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "礼金（敷引）",
                vec![UnitId::OneShotYen],
                &InputId::KeyMoneyUnit.to_string(),
                "初期費用として請求されるメジャーなものその1。だいたい賃料の1か月分。入居時にオーナーへ感謝の気持ちを込めて払うものだったらしいが、企業が管理している物件だと形骸化している気がしてならない。早く埋めたい物件では真っ先に安くされる。オーナーは礼金0にしているのに、仲介業者が増やして中抜きする場合もあるとか無いとか。",
                &InputId::KeyMoney.to_string(),
                form_items::InputValidation {
                    default_value: Some(50000),
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "仲介手数料",
                vec![UnitId::OneShotYen],
                &InputId::BrokerageFeeUnit.to_string(),
                "初期費用として請求されるメジャーなものその2。だいたい賃料の1か月分。こちらは仲介業者へ感謝の気持ちを込めて払うもの。仲介業者も当然お仕事なのでタダでは動けない。ただし、オーナーから成約報酬として十分な額がある場合、仲介手数料は0になる。据え置くこともある。",
                &InputId::BrokerageFee.to_string(),
                form_items::InputValidation {
                    default_value: Some(50000),
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "フリーレント",
                vec![UnitId::Month],
                &InputId::FreeRentPeriodUnit.to_string(),
                "この期間は賃料が発生しない。唯一増えると嬉しい値。ただし共益費はかかる点、期間中の退去は違約金が発生しうる点に注意。",
                &InputId::FreeRentPeriod.to_string(),
                form_items::InputValidation {
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "保証料",
                vec![UnitId::OneShotYen, UnitId::MonthlyYen, UnitId::EveryContractYen],
                &InputId::GuaranteeFeeUnit.to_string(),
                "保証会社必須の場合に発生することもある。初回に一括で家賃の50%というパターンがあったり。毎月家賃合計の1%というパターンもある。契約更新時にまた払うこともあるが、初回よりかなり安くなるのが通例。最近は保証会社利用必須の物件が増えており、しょうがない気はするがコストが余計にかさむ。",
                &InputId::GuaranteeFee.to_string(),
                form_items::InputValidation {
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "24時間サポート",
                vec![UnitId::MonthlyYen, UnitId::EveryContractYen, UnitId::OneShotYen],
                &InputId::SupportFeeUnit.to_string(),
                "大手マンションだと発生しうる税金その1。なぜか強制なことが多いが、営業時間外どころか1回も使わないことが多い。任意加入の場合はちゃんと考えて選択しよう。",
                &InputId::SupportFee.to_string(),
                form_items::InputValidation {
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "町内会費",
                vec![UnitId::MonthlyYen, UnitId::EveryContractYen, UnitId::OneShotYen],
                &InputId::AssociationMembershipFeeUnit.to_string(),
                "町内会に貢献するための費用。なぜか強制支払い。賃貸だと通常ないため、これがある部屋は何か怪しい。",
                &InputId::AssociationMembershipFee.to_string(),
                form_items::InputValidation {
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "保証委託契約時事務手数料",
                vec![UnitId::OneShotYen],
                &InputId::GuaranteeAdministrativeFeeUnit.to_string(),
                "保証会社を使う際に初回だけ発生する費用。事務処理という名目だが、妙に高い場合が多い。実際に何をしているのかは分からないので、言い値を払うことになる。",
                &InputId::GuaranteeAdministrativeFee.to_string(),
                form_items::InputValidation {
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "保険料",
                vec![UnitId::EveryContractYen, UnitId::OneShotYen],
                &InputId::InsuranceFeeUnit.to_string(),
                "火災保険など。だいたい選択の余地は無い。せめて会社やプランを選ばせてほしいところ。良心的な契約では複数社から選べたりするが、いずれにせよ選択肢は狭い。",
                &InputId::InsuranceFee.to_string(),
                form_items::InputValidation {
                    default_value: Some(10000),
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "自転車・バイク置き場使用料",
                vec![UnitId::EveryContractYen, UnitId::MonthlyYen, UnitId::OneShotYen],
                &InputId::BicycleSpaceFeeUnit.to_string(),
                "二輪を持っている人には必要な費用。契約更新に合わせてこちらも更新されることが多い。独立した契約になることで、不要な人は払わなくて良い。良心的。",
                &InputId::BicycleSpaceFee.to_string(),
                form_items::InputValidation {
                    default_value: Some(3000),
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "自動車置き場費用",
                vec![UnitId::EveryContractYen, UnitId::MonthlyYen, UnitId::OneShotYen],
                &InputId::CarSpaceFeeUnit.to_string(),
                "自転車などと同様、通常契約するかを選べる。専有するスペースが段違いなので、都心では二輪と比べ凄まじい費用がかかる。そもそも部屋数に対し用意されている数が少ないため、自転車の100倍、バイクの10倍の費用がかかることも。",
                &InputId::CarSpaceFee.to_string(),
                form_items::InputValidation {
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "鍵交換費用",
                vec![UnitId::OneShotYen],
                &InputId::KeyChangeFeeUnit.to_string(),
                "新築以外では入居時にかかることが多い謎費用の1つ。「セキュリティを守るため、鍵を変える必要があるんですよねー」ってそれオーナー側の負担でやるべきことでは。そもそも選択権がなく強制的に徴収される。もしオーナーが負担して交換してくれていたら感謝しよう。交換がされてなかったら……気をつけよう。",
                &InputId::KeyChangeFee.to_string(),
                form_items::InputValidation {
                    default_value: Some(5000),
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "退去時清掃費用",
                vec![UnitId::OneShotYen],
                &InputId::CleaningFeeUnit.to_string(),
                "敷金から引かれるものとは別に、固定でかかる費用。エアコン清掃などの名目で書かれているが、なぜか敷金と同じ括りにはならない。実質敷引き。原状回復の基準は明確になってきているので、こういう所で法律逃れの小銭集めをしているのかもしれない。大体特約に書かれているので、契約時には注意しよう。",
                &InputId::CleaningFee.to_string(),
                form_items::InputValidation {
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "更新料",
                vec![UnitId::EveryContractUpdateYen],
                &InputId::ContractUpdateFeeUnit.to_string(),
                "契約更新時に発生する費用。長く住んでいると、何故か更新料が発生し追い出し圧力をかけてくる。初期費用が美味しいからではないかと推測される。関東に多い風習。だいたい賃料の1か月。「更新事務手数料」となっている場合もあり、なぜか新しい人を入れるほうが事務作業がてこずらないという事態になっている。",
                &InputId::ContractUpdateFee.to_string(),
                form_items::InputValidation {
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "契約期間",
                vec![UnitId::Month],
                &InputId::ContractPeriodUnit.to_string(),
                "賃貸契約を更新するまでの期間。通常2年だが、定期借家契約だと色々。",
                &InputId::ContractPeriod.to_string(),
                form_items::InputValidation {
                    default_value: Some(24),
                    ..Default::default()
                },
            ),
            form_items::RentItem::new(
                "居住期間",
                vec![UnitId::Month],
                &InputId::LeasePeriodUnit.to_string(),
                "何か月住む想定か。これによって礼金のダメージなどが変わってくる。基本的に長く住むほど安上がりだが、いい物件があれば早い段階で引っ越す方が安上がりな場合もある。",
                &InputId::LeasePeriod.to_string(),
                form_items::InputValidation {
                    default_value: Some(24),
                    ..Default::default()
                },
            ),
        ]
    )
}
