use types::*;
use fix40::fields::*;

pub struct Header {
    pub sender_comp_id: FIXString,
    pub target_comp_id: FIXString,
    pub on_behalf_of_comp_id: Option<FIXString>,
    pub deliver_to_comp_id: Option<FIXString>,
    pub secure_data_len: Option<Length>,
    pub secure_data: Option<Data>,
    pub msg_seq_num: FIXInt,
    pub sender_sub_id: Option<FIXString>,
    pub target_sub_id: Option<FIXString>,
    pub on_behalf_of_sub_id: Option<FIXString>,
    pub deliver_to_sub_id: Option<FIXString>,
    pub poss_dup_flag: Option<PossDupFlag>,
    pub poss_resend: Option<FIXString>,
    pub orig_sending_time: Option<UTCTimestamp>
}

pub struct Message {
    /// Standard message header.
    pub header: Header,
    /// Message body
    pub body: MessageBody,
}

pub enum MessageBody {
    /// Heartbeat message. Message code: 0
    Heartbeat {
        test_req_id: Option<FIXString>
    },

    /// TestRequest message. Message code: 1
    TestRequest {
        test_req_id: FIXString
    },

    /// ResendRequest message. Message code: 2
    ResendRequest {
        begin_seq_no: FIXInt,
        end_seq_no: FIXInt
    },

    /// Reject message. Message code: 3
    Reject {
        ref_seq_num: FIXInt,
        text: Option<FIXString>
    },

    /// SequenceReset message. Message code: 4
    SequenceReset {
        gap_fill_flag: Option<GapFillFlag>,
        new_seq_no: FIXInt
    },

    /// Logout message. Message code: 5
    Logout {
        text: Option<FIXString>
    },

    /// IOI message. Message code: 6
    IOI {
        io_iid: FIXInt,
        ioi_trans_type: IOITransType,
        ioi_ref_id: Option<FIXInt>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        side: Side,
        ioi_shares: IOIShares,
        price: Option<FIXFloat>,
        currency: Option<FIXString>,
        valid_until_time: Option<UTCTimestamp>,
        ioi_qlty_ind: Option<IOIQltyInd>,
        ioi_oth_svc: Option<IOIOthSvc>,
        ioi_natural_flag: Option<IOINaturalFlag>,
        ioi_qualifier: Option<IOIQualifier>,
        text: Option<FIXString>
    },

    /// Advertisement message. Message code: 7
    Advertisement {
        adv_id: FIXInt,
        adv_trans_type: AdvTransType,
        adv_ref_id: Option<FIXInt>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        adv_side: AdvSide,
        shares: FIXInt,
        price: Option<FIXFloat>,
        currency: Option<FIXString>,
        transact_time: Option<UTCTimestamp>,
        text: Option<FIXString>
    },

    /// ExecutionReport message. Message code: 8
    ExecutionReport {
        order_id: FIXString,
        cl_ord_id: Option<FIXString>,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        list_id: Option<FIXString>,
        exec_id: FIXInt,
        exec_trans_type: ExecTransType,
        exec_ref_id: Option<FIXInt>,
        ord_status: OrdStatus,
        ord_rej_reason: Option<OrdRejReason>,
        account: Option<FIXString>,
        settlmnt_typ: Option<SettlmntTyp>,
        fut_sett_date: Option<UTCDateOnly>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        side: Side,
        order_qty: FIXInt,
        ord_type: Option<OrdType>,
        price: Option<FIXFloat>,
        stop_px: Option<FIXFloat>,
        currency: Option<FIXString>,
        time_in_force: Option<TimeInForce>,
        expire_time: Option<UTCTimestamp>,
        exec_inst: Option<ExecInst>,
        rule80_a: Option<Rule80A>,
        last_shares: FIXInt,
        last_px: FIXFloat,
        last_mkt: Option<FIXString>,
        last_capacity: Option<LastCapacity>,
        cum_qty: FIXInt,
        avg_px: FIXFloat,
        trade_date: Option<UTCDateOnly>,
        transact_time: Option<UTCTimestamp>,
        report_to_exch: Option<ReportToExch>,
        commission: Option<FIXFloat>,
        comm_type: Option<CommType>,
        misc_fee_amt: Option<FIXFloat>,
        misc_fee_curr: Option<FIXString>,
        misc_fee_type: Option<MiscFeeType>,
        net_money: Option<FIXFloat>,
        settl_curr_amt: Option<FIXFloat>,
        settl_currency: Option<FIXString>,
        text: Option<FIXString>
    },

    /// OrderCancelReject message. Message code: 9
    OrderCancelReject {
        order_id: FIXString,
        cl_ord_id: FIXString,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        list_id: Option<FIXString>,
        cxl_rej_reason: Option<CxlRejReason>,
        text: Option<FIXString>
    },

    /// Logon message. Message code: A
    Logon {
        encrypt_method: EncryptMethod,
        heart_bt_int: FIXInt,
        raw_data_length: Option<Length>,
        raw_data: Option<Data>
    },

    /// News message. Message code: B
    News {
        orig_time: Option<UTCTimestamp>,
        urgency: Option<Urgency>,
        relatd_sym: Option<FIXString>,
        lines_of_text: FIXInt,
        text: FIXString,
        raw_data_length: Option<Length>,
        raw_data: Option<Data>
    },

    /// Email message. Message code: C
    Email {
        email_type: EmailType,
        orig_time: Option<UTCTimestamp>,
        relatd_sym: Option<FIXString>,
        order_id: Option<FIXString>,
        cl_ord_id: Option<FIXString>,
        lines_of_text: FIXInt,
        text: FIXString,
        raw_data_length: Option<Length>,
        raw_data: Option<Data>
    },

    /// NewOrderSingle message. Message code: D
    NewOrderSingle {
        cl_ord_id: FIXString,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        account: Option<FIXString>,
        settlmnt_typ: Option<SettlmntTyp>,
        fut_sett_date: Option<UTCDateOnly>,
        handl_inst: HandlInst,
        exec_inst: Option<ExecInst>,
        min_qty: Option<FIXInt>,
        max_floor: Option<FIXInt>,
        ex_destination: Option<ExDestination>,
        process_code: Option<ProcessCode>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        prev_close_px: Option<FIXFloat>,
        side: Side,
        locate_reqd: Option<LocateReqd>,
        order_qty: FIXInt,
        ord_type: OrdType,
        price: Option<FIXFloat>,
        stop_px: Option<FIXFloat>,
        currency: Option<FIXString>,
        io_iid: Option<FIXInt>,
        quote_id: Option<FIXString>,
        time_in_force: Option<TimeInForce>,
        expire_time: Option<UTCTimestamp>,
        commission: Option<FIXFloat>,
        comm_type: Option<CommType>,
        rule80_a: Option<Rule80A>,
        forex_req: Option<ForexReq>,
        settl_currency: Option<FIXString>,
        text: Option<FIXString>
    },

    /// NewOrderList message. Message code: E
    NewOrderList {
        list_id: FIXString,
        wave_no: Option<FIXString>,
        list_seq_no: FIXInt,
        list_no_ords: FIXInt,
        list_exec_inst: Option<FIXString>,
        cl_ord_id: FIXString,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        account: Option<FIXString>,
        settlmnt_typ: Option<SettlmntTyp>,
        fut_sett_date: Option<UTCDateOnly>,
        handl_inst: HandlInst,
        exec_inst: Option<ExecInst>,
        min_qty: Option<FIXInt>,
        max_floor: Option<FIXInt>,
        ex_destination: Option<ExDestination>,
        process_code: Option<ProcessCode>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        prev_close_px: Option<FIXFloat>,
        side: Side,
        locate_reqd: Option<LocateReqd>,
        order_qty: FIXInt,
        ord_type: OrdType,
        price: Option<FIXFloat>,
        stop_px: Option<FIXFloat>,
        currency: Option<FIXString>,
        time_in_force: Option<TimeInForce>,
        expire_time: Option<UTCTimestamp>,
        commission: Option<FIXFloat>,
        comm_type: Option<CommType>,
        rule80_a: Option<Rule80A>,
        forex_req: Option<ForexReq>,
        settl_currency: Option<FIXString>,
        text: Option<FIXString>
    },

    /// OrderCancelRequest message. Message code: F
    OrderCancelRequest {
        orig_cl_ord_id: FIXString,
        order_id: Option<FIXString>,
        cl_ord_id: FIXString,
        list_id: Option<FIXString>,
        cxl_type: CxlType,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        side: Side,
        order_qty: FIXInt,
        text: Option<FIXString>
    },

    /// OrderCancelReplaceRequest message. Message code: G
    OrderCancelReplaceRequest {
        order_id: Option<FIXString>,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        orig_cl_ord_id: FIXString,
        cl_ord_id: FIXString,
        list_id: Option<FIXString>,
        account: Option<FIXString>,
        settlmnt_typ: Option<SettlmntTyp>,
        fut_sett_date: Option<UTCDateOnly>,
        handl_inst: HandlInst,
        exec_inst: Option<ExecInst>,
        min_qty: Option<FIXInt>,
        max_floor: Option<FIXInt>,
        ex_destination: Option<ExDestination>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        side: Side,
        order_qty: FIXInt,
        ord_type: OrdType,
        price: Option<FIXFloat>,
        stop_px: Option<FIXFloat>,
        currency: Option<FIXString>,
        time_in_force: Option<TimeInForce>,
        expire_time: Option<UTCTimestamp>,
        commission: Option<FIXFloat>,
        comm_type: Option<CommType>,
        rule80_a: Option<Rule80A>,
        forex_req: Option<ForexReq>,
        settl_currency: Option<FIXString>,
        text: Option<FIXString>
    },

    /// OrderStatusRequest message. Message code: H
    OrderStatusRequest {
        order_id: Option<FIXString>,
        cl_ord_id: FIXString,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        side: Side
    },

    /// Allocation message. Message code: J
    Allocation {
        alloc_id: FIXInt,
        alloc_trans_type: AllocTransType,
        ref_alloc_id: Option<FIXInt>,
        cl_ord_id: FIXString,
        order_id: Option<FIXString>,
        list_id: Option<FIXString>,
        wave_no: Option<FIXString>,
        exec_id: Option<FIXInt>,
        last_shares: Option<FIXInt>,
        last_px: Option<FIXFloat>,
        last_mkt: Option<FIXString>,
        side: Side,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        shares: FIXInt,
        avg_px: FIXFloat,
        currency: Option<FIXString>,
        avg_prx_precision: Option<FIXInt>,
        trade_date: UTCDateOnly,
        transact_time: Option<UTCTimestamp>,
        settlmnt_typ: Option<SettlmntTyp>,
        fut_sett_date: Option<UTCDateOnly>,
        net_money: Option<FIXFloat>,
        misc_fee_amt: Option<FIXFloat>,
        misc_fee_curr: Option<FIXString>,
        misc_fee_type: Option<MiscFeeType>,
        settl_curr_amt: Option<FIXFloat>,
        settl_currency: Option<FIXString>,
        open_close: Option<FIXString>,
        text: Option<FIXString>,
        alloc_account: FIXString,
        alloc_shares: FIXInt,
        process_code: Option<ProcessCode>,
        exec_broker: Option<FIXString>,
        client_id: Option<FIXString>,
        commission: Option<FIXFloat>,
        comm_type: Option<CommType>,
        no_dlvy_inst: Option<FIXInt>,
        broker_of_credit: Option<FIXString>,
        dlvy_inst: Option<FIXString>
    },

    /// ListCancelRequest message. Message code: K
    ListCancelRequest {
        list_id: FIXString,
        wave_no: Option<FIXString>,
        text: Option<FIXString>
    },

    /// ListExecute message. Message code: L
    ListExecute {
        list_id: FIXString,
        wave_no: Option<FIXString>,
        text: Option<FIXString>
    },

    /// ListStatusRequest message. Message code: M
    ListStatusRequest {
        list_id: FIXString,
        wave_no: Option<FIXString>,
        text: Option<FIXString>
    },

    /// ListStatus message. Message code: N
    ListStatus {
        list_id: FIXString,
        wave_no: Option<FIXString>,
        no_rpts: FIXInt,
        rpt_seq: FIXInt,
        cl_ord_id: FIXString,
        cum_qty: FIXInt,
        cxl_qty: FIXInt,
        avg_px: FIXFloat
    },

    /// AllocationInstructionAck message. Message code: P
    AllocationInstructionAck {
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        alloc_id: FIXInt,
        trade_date: UTCDateOnly,
        transact_time: Option<UTCTimestamp>,
        alloc_status: AllocStatus,
        alloc_rej_code: Option<AllocRejCode>,
        text: Option<FIXString>
    },

    /// DontKnowTrade message. Message code: Q
    DontKnowTrade {
        order_id: Option<FIXString>,
        exec_id: Option<FIXInt>,
        dk_reason: DKReason,
        symbol: FIXString,
        side: Side,
        order_qty: FIXInt,
        last_shares: FIXInt,
        last_px: FIXFloat,
        text: Option<FIXString>
    },

    /// QuoteRequest message. Message code: R
    QuoteRequest {
        quote_req_id: FIXString,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        prev_close_px: Option<FIXFloat>,
        side: Option<Side>,
        order_qty: Option<FIXInt>
    },

    /// Quote message. Message code: S
    Quote {
        quote_req_id: Option<FIXString>,
        quote_id: FIXString,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        issuer: Option<FIXString>,
        security_desc: Option<FIXString>,
        bid_px: FIXFloat,
        offer_px: Option<FIXFloat>,
        bid_size: Option<FIXInt>,
        offer_size: Option<FIXInt>,
        valid_until_time: Option<UTCTimestamp>
    }
}