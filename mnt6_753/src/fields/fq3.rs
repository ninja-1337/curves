use crate::{fq::Fq, FQ_ONE};
use ark_ff::{
    biginteger::BigInteger768 as BigInteger,
    field_new,
    fields::fp3::{Fp3, Fp3Parameters},
};

pub type Fq3 = Fp3<Fq3Parameters>;

pub struct Fq3Parameters;

impl Fp3Parameters for Fq3Parameters {
    type Fp = Fq;

    #[rustfmt::skip]
    const NONRESIDUE: Fq = field_new!(Fq, BigInteger([
        5145524327033718740,
        14149824967095184544,
        5159730833497260295,
        3902941467692815387,
        15830098551216085679,
        8665641533746801158,
        17502192300007146323,
        14483698255198590748,
        546300946688995976,
        4331975528992054828,
        5311428878520309260,
        495362057711802,
    ]));

    const TWO_ADICITY: u32 = 30;

    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: &'static [u64] = &[
        15439605736802142541,
        18190868848461853149,
        6220121510046940818,
        10310485528612680366,
        5032137869959796540,
        3943048799800510054,
        1971151279016362045,
        6096644900171872841,
        12908407994230849218,
        4163225373804228290,
        10382959950522770522,
        9008828410264446883,
        18411821899404157689,
        12386199240837247984,
        13370099281150720481,
        11909278545073807560,
        5964354403900302648,
        15347506722065009035,
        7045354120681109597,
        14294096902719509929,
        6180325033003959541,
        14381489272445870003,
        18159920240207503954,
        17487026929061632528,
        12314108197538755669,
        12116872703077811769,
        3401400733784294722,
        13905351619889935522,
        10972472942574358218,
        6104159581753028261,
        4690139121547787552,
        4880965491878697414,
        1926648890365125214,
        13532564555356297305,
        3114545746551080,
        0,
    ];

    #[rustfmt::skip]
    const QUADRATIC_NONRESIDUE_TO_T: (Fq, Fq, Fq) = (
        field_new!(Fq, BigInteger([
            2456656400918202012,
            7503386575313625620,
            1014314685003569848,
            10473903647598823719,
            15893393002146336511,
            8418203974290622500,
            9017296731996077946,
            2923126592994124774,
            9368756030960215800,
            17344552888362241070,
            10938255746876359306,
            107029542386399,
        ])),
        field_new!(Fq, BigInteger([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])),
        field_new!(Fq, BigInteger([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])),
    );

    // Coefficients for the Frobenius automorphism.
    // c1[0] = 1,
    // c1[1] = 24129022407817241407134263419936114379815707076943508280977368156625538709102831814843582780138963119807143081677569721953561801075623741378629346409604471234573396989178424163772589090105392407118197799904755622897541183052132
    // c1[2] = 17769468560101711995209951371304522748355002843010440790806134764399814103468274958215310983651375801610927890210888755369611256415970113691066895445191924931148019336171640277697829047741006062493737919155152541323243293107868,
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP3_C1: &'static [Fq] = &[
        FQ_ONE,
        field_new!(Fq, BigInteger([
            7739145380395648640,
            1403348385939055902,
            11220424057264707228,
            4567962295300549271,
            5929583493640677751,
            17618207486530478833,
            16600462137977359741,
            16551719371247820635,
            12057922785354578416,
            13022559182829558162,
            13308285686168533250,
            313705269181021,
        ])),
        field_new!(Fq, BigInteger([
            12973180669431253567,
            17038664486452692616,
            11034024317238370177,
            7712681843988565810,
            4725787734130647531,
            2175028350442404679,
            9323639551697167751,
            14465264105466053583,
            8569442212929419360,
            17553812953652473294,
            13991744086792172309,
            48577617831792,
        ])),
    ];

    // c2 = {c1[0], c1[2], c1[1]}
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP3_C2: &'static [Fq] = &[
        FQ_ONE,
        Self::FROBENIUS_COEFF_FP3_C1[2],
        Self::FROBENIUS_COEFF_FP3_C1[1],
    ];
}
