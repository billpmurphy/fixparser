use types::*;
use fix42::fields::*;

pub struct Header {
    pub begin_string: FIXString,
    pub body_length: FIXInt,
    pub msg_type: MsgType,
    pub sender_comp_id: FIXString,
    pub target_comp_id: FIXString,
    pub on_behalf_of_comp_id: Option<FIXString>,
    pub deliver_to_comp_id: Option<FIXString>,
    pub secure_data_len: Option<Length>,
    pub secure_data: Option<Data>,
    pub msg_seq_num: FIXInt,
    pub sender_sub_id: Option<FIXString>,
    pub sender_location_id: Option<FIXString>,
    pub target_sub_id: Option<FIXString>,
    pub target_location_id: Option<FIXString>,
    pub on_behalf_of_sub_id: Option<FIXString>,
    pub on_behalf_of_location_id: Option<FIXString>,
    pub deliver_to_sub_id: Option<FIXString>,
    pub deliver_to_location_id: Option<FIXString>,
    pub poss_dup_flag: Option<PossDupFlag>,
    pub poss_resend: Option<PossResend>,
    pub sending_time: UTCTimestamp,
    pub orig_sending_time: Option<UTCTimestamp>,
    pub xml_data_len: Option<Length>,
    pub xml_data: Option<Data>,
    pub message_encoding: Option<MessageEncoding>,
    pub last_msg_seq_num_processed: Option<FIXInt>,
    pub on_behalf_of_sending_time: Option<UTCTimestamp>
}

pub struct Trailer {
    pub begin_string: FIXString,
    pub body_length: FIXInt,
    pub msg_type: MsgType,
    pub sender_comp_id: FIXString,
    pub target_comp_id: FIXString,
    pub on_behalf_of_comp_id: Option<FIXString>,
    pub deliver_to_comp_id: Option<FIXString>,
    pub secure_data_len: Option<Length>,
    pub secure_data: Option<Data>,
    pub msg_seq_num: FIXInt,
    pub sender_sub_id: Option<FIXString>,
    pub sender_location_id: Option<FIXString>,
    pub target_sub_id: Option<FIXString>,
    pub target_location_id: Option<FIXString>,
    pub on_behalf_of_sub_id: Option<FIXString>,
    pub on_behalf_of_location_id: Option<FIXString>,
    pub deliver_to_sub_id: Option<FIXString>,
    pub deliver_to_location_id: Option<FIXString>,
    pub poss_dup_flag: Option<PossDupFlag>,
    pub poss_resend: Option<PossResend>,
    pub sending_time: UTCTimestamp,
    pub orig_sending_time: Option<UTCTimestamp>,
    pub xml_data_len: Option<Length>,
    pub xml_data: Option<Data>,
    pub message_encoding: Option<MessageEncoding>,
    pub last_msg_seq_num_processed: Option<FIXInt>,
    pub on_behalf_of_sending_time: Option<UTCTimestamp>
}

pub struct Message {
    pub header: Header,
    pub trailer: Trailer,
    pub body: MessageBody
}

pub enum MessageBody {
    Heartbeat {
        test_req_id: Option<FIXString>
    },
    TestRequest {
        test_req_id: FIXString
    },
    ResendRequest {
        begin_seq_no: FIXInt,
        end_seq_no: FIXInt
    },
    Reject {
        ref_seq_num: FIXInt,
        ref_tag_id: Option<FIXInt>,
        ref_msg_type: Option<FIXString>,
        session_reject_reason: Option<SessionRejectReason>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    SequenceReset {
        gap_fill_flag: Option<GapFillFlag>,
        new_seq_no: FIXInt
    },
    Logout {
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    IOI {
        io_iid: FIXString,
        ioi_trans_type: IOITransType,
        ioi_ref_id: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        side: Side,
        ioi_shares: IOIShares,
        price: Option<Price>,
        currency: Option<Currency>,
        valid_until_time: Option<UTCTimestamp>,
        ioi_qlty_ind: Option<IOIQltyInd>,
        ioi_natural_flag: Option<IOINaturalFlag>,
        no_ioi_qualifiers: Option<FIXInt>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>,
        transact_time: Option<UTCTimestamp>,
        url_link: Option<FIXString>,
        no_routing_i_ds: Option<FIXInt>,
        spread_to_benchmark: Option<PriceOffset>,
        benchmark: Option<Benchmark>
    },
    Advertisement {
        adv_id: FIXString,
        adv_trans_type: AdvTransType,
        adv_ref_id: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        adv_side: AdvSide,
        shares: Qty,
        price: Option<Price>,
        currency: Option<Currency>,
        trade_date: Option<FIXString>,
        transact_time: Option<UTCTimestamp>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>,
        url_link: Option<FIXString>,
        last_mkt: Option<FIXString>,
        trading_session_id: Option<FIXString>
    },
    ExecutionReport {
        order_id: FIXString,
        secondary_order_id: Option<FIXString>,
        cl_ord_id: Option<FIXString>,
        orig_cl_ord_id: Option<FIXString>,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        no_contra_brokers: Option<FIXInt>,
        list_id: Option<FIXString>,
        exec_id: FIXString,
        exec_trans_type: ExecTransType,
        exec_ref_id: Option<FIXString>,
        exec_type: ExecType,
        ord_status: OrdStatus,
        ord_rej_reason: Option<OrdRejReason>,
        exec_restatement_reason: Option<ExecRestatementReason>,
        account: Option<FIXString>,
        settlmnt_typ: Option<SettlmntTyp>,
        fut_sett_date: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        side: Side,
        order_qty: Option<Qty>,
        cash_order_qty: Option<Qty>,
        ord_type: Option<OrdType>,
        price: Option<Price>,
        stop_px: Option<Price>,
        peg_difference: Option<PriceOffset>,
        discretion_inst: Option<DiscretionInst>,
        discretion_offset: Option<PriceOffset>,
        currency: Option<Currency>,
        compliance_id: Option<FIXString>,
        solicited_flag: Option<SolicitedFlag>,
        time_in_force: Option<TimeInForce>,
        effective_time: Option<UTCTimestamp>,
        expire_date: Option<FIXString>,
        expire_time: Option<UTCTimestamp>,
        exec_inst: Option<ExecInst>,
        rule80_a: Option<Rule80A>,
        last_shares: Option<Qty>,
        last_px: Option<Price>,
        last_spot_rate: Option<Price>,
        last_forward_points: Option<PriceOffset>,
        last_mkt: Option<FIXString>,
        trading_session_id: Option<FIXString>,
        last_capacity: Option<LastCapacity>,
        leaves_qty: Qty,
        cum_qty: Qty,
        avg_px: Price,
        day_order_qty: Option<Qty>,
        day_cum_qty: Option<Qty>,
        day_avg_px: Option<Price>,
        gt_booking_inst: Option<GTBookingInst>,
        trade_date: Option<FIXString>,
        transact_time: Option<UTCTimestamp>,
        report_to_exch: Option<ReportToExch>,
        commission: Option<Amt>,
        comm_type: Option<CommType>,
        gross_trade_amt: Option<Amt>,
        settl_curr_amt: Option<Amt>,
        settl_currency: Option<Currency>,
        settl_curr_fx_rate: Option<FIXFloat>,
        settl_curr_fx_rate_calc: Option<SettlCurrFxRateCalc>,
        handl_inst: Option<HandlInst>,
        min_qty: Option<Qty>,
        max_floor: Option<Qty>,
        open_close: Option<OpenClose>,
        max_show: Option<Qty>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>,
        fut_sett_date2: Option<FIXString>,
        order_qty2: Option<Qty>,
        clearing_firm: Option<FIXString>,
        clearing_account: Option<FIXString>,
        multi_leg_reporting_type: Option<MultiLegReportingType>
    },
    OrderCancelReject {
        order_id: FIXString,
        secondary_order_id: Option<FIXString>,
        cl_ord_id: FIXString,
        orig_cl_ord_id: FIXString,
        ord_status: OrdStatus,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        list_id: Option<FIXString>,
        account: Option<FIXString>,
        transact_time: Option<UTCTimestamp>,
        cxl_rej_response_to: CxlRejResponseTo,
        cxl_rej_reason: Option<CxlRejReason>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    Logon {
        encrypt_method: EncryptMethod,
        heart_bt_int: FIXInt,
        raw_data_length: Option<Length>,
        raw_data: Option<Data>,
        reset_seq_num_flag: Option<ResetSeqNumFlag>,
        max_message_size: Option<FIXInt>,
        no_msg_types: Option<FIXInt>
    },
    News {
        orig_time: Option<UTCTimestamp>,
        urgency: Option<Urgency>,
        headline: FIXString,
        encoded_headline_len: Option<Length>,
        encoded_headline: Option<Data>,
        no_routing_i_ds: Option<FIXInt>,
        no_related_sym: Option<FIXInt>,
        lines_of_text: FIXInt,
        url_link: Option<FIXString>,
        raw_data_length: Option<Length>,
        raw_data: Option<Data>
    },
    Email {
        email_thread_id: FIXString,
        email_type: EmailType,
        orig_time: Option<UTCTimestamp>,
        subject: FIXString,
        encoded_subject_len: Option<Length>,
        encoded_subject: Option<Data>,
        no_routing_i_ds: Option<FIXInt>,
        no_related_sym: Option<FIXInt>,
        order_id: Option<FIXString>,
        cl_ord_id: Option<FIXString>,
        lines_of_text: FIXInt,
        raw_data_length: Option<Length>,
        raw_data: Option<Data>
    },
    NewOrderSingle {
        cl_ord_id: FIXString,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        account: Option<FIXString>,
        no_allocs: Option<FIXInt>,
        settlmnt_typ: Option<SettlmntTyp>,
        fut_sett_date: Option<FIXString>,
        handl_inst: HandlInst,
        exec_inst: Option<ExecInst>,
        min_qty: Option<Qty>,
        max_floor: Option<Qty>,
        ex_destination: Option<FIXString>,
        no_trading_sessions: Option<FIXInt>,
        process_code: Option<ProcessCode>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        prev_close_px: Option<Price>,
        side: Side,
        locate_reqd: Option<LocateReqd>,
        transact_time: UTCTimestamp,
        order_qty: Option<Qty>,
        cash_order_qty: Option<Qty>,
        ord_type: OrdType,
        price: Option<Price>,
        stop_px: Option<Price>,
        currency: Option<Currency>,
        compliance_id: Option<FIXString>,
        solicited_flag: Option<SolicitedFlag>,
        io_iid: Option<FIXString>,
        quote_id: Option<FIXString>,
        time_in_force: Option<TimeInForce>,
        effective_time: Option<UTCTimestamp>,
        expire_date: Option<FIXString>,
        expire_time: Option<UTCTimestamp>,
        gt_booking_inst: Option<GTBookingInst>,
        commission: Option<Amt>,
        comm_type: Option<CommType>,
        rule80_a: Option<Rule80A>,
        forex_req: Option<ForexReq>,
        settl_currency: Option<Currency>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>,
        fut_sett_date2: Option<FIXString>,
        order_qty2: Option<Qty>,
        open_close: Option<OpenClose>,
        covered_or_uncovered: Option<CoveredOrUncovered>,
        customer_or_firm: Option<CustomerOrFirm>,
        max_show: Option<Qty>,
        peg_difference: Option<PriceOffset>,
        discretion_inst: Option<DiscretionInst>,
        discretion_offset: Option<PriceOffset>,
        clearing_firm: Option<FIXString>,
        clearing_account: Option<FIXString>
    },
    NewOrderList {
        list_id: FIXString,
        bid_id: Option<FIXString>,
        client_bid_id: Option<FIXString>,
        prog_rpt_reqs: Option<ProgRptReqs>,
        bid_type: FIXInt,
        prog_period_interval: Option<FIXInt>,
        list_exec_inst_type: Option<ListExecInstType>,
        list_exec_inst: Option<FIXString>,
        encoded_list_exec_inst_len: Option<Length>,
        encoded_list_exec_inst: Option<Data>,
        tot_no_orders: FIXInt,
        no_orders: FIXInt
    },
    OrderCancelRequest {
        orig_cl_ord_id: FIXString,
        order_id: Option<FIXString>,
        cl_ord_id: FIXString,
        list_id: Option<FIXString>,
        account: Option<FIXString>,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        side: Side,
        transact_time: UTCTimestamp,
        order_qty: Option<Qty>,
        cash_order_qty: Option<Qty>,
        compliance_id: Option<FIXString>,
        solicited_flag: Option<SolicitedFlag>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    OrderCancelReplaceRequest {
        order_id: Option<FIXString>,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        orig_cl_ord_id: FIXString,
        cl_ord_id: FIXString,
        list_id: Option<FIXString>,
        account: Option<FIXString>,
        no_allocs: Option<FIXInt>,
        settlmnt_typ: Option<SettlmntTyp>,
        fut_sett_date: Option<FIXString>,
        handl_inst: HandlInst,
        exec_inst: Option<ExecInst>,
        min_qty: Option<Qty>,
        max_floor: Option<Qty>,
        ex_destination: Option<FIXString>,
        no_trading_sessions: Option<FIXInt>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        side: Side,
        transact_time: UTCTimestamp,
        order_qty: Option<Qty>,
        cash_order_qty: Option<Qty>,
        ord_type: OrdType,
        price: Option<Price>,
        stop_px: Option<Price>,
        peg_difference: Option<PriceOffset>,
        discretion_inst: Option<DiscretionInst>,
        discretion_offset: Option<PriceOffset>,
        compliance_id: Option<FIXString>,
        solicited_flag: Option<SolicitedFlag>,
        currency: Option<Currency>,
        time_in_force: Option<TimeInForce>,
        effective_time: Option<UTCTimestamp>,
        expire_date: Option<FIXString>,
        expire_time: Option<UTCTimestamp>,
        gt_booking_inst: Option<GTBookingInst>,
        commission: Option<Amt>,
        comm_type: Option<CommType>,
        rule80_a: Option<Rule80A>,
        forex_req: Option<ForexReq>,
        settl_currency: Option<Currency>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>,
        fut_sett_date2: Option<FIXString>,
        order_qty2: Option<Qty>,
        open_close: Option<OpenClose>,
        covered_or_uncovered: Option<CoveredOrUncovered>,
        customer_or_firm: Option<CustomerOrFirm>,
        max_show: Option<Qty>,
        locate_reqd: Option<LocateReqd>,
        clearing_firm: Option<FIXString>,
        clearing_account: Option<FIXString>
    },
    OrderStatusRequest {
        order_id: Option<FIXString>,
        cl_ord_id: FIXString,
        client_id: Option<FIXString>,
        account: Option<FIXString>,
        exec_broker: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        side: Side
    },
    Allocation {
        alloc_id: FIXString,
        alloc_trans_type: AllocTransType,
        ref_alloc_id: Option<FIXString>,
        alloc_link_id: Option<FIXString>,
        alloc_link_type: Option<AllocLinkType>,
        no_orders: Option<FIXInt>,
        no_execs: Option<FIXInt>,
        side: Side,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        shares: Qty,
        last_mkt: Option<FIXString>,
        trading_session_id: Option<FIXString>,
        avg_px: Price,
        currency: Option<Currency>,
        avg_prx_precision: Option<FIXInt>,
        trade_date: FIXString,
        transact_time: Option<UTCTimestamp>,
        settlmnt_typ: Option<SettlmntTyp>,
        fut_sett_date: Option<FIXString>,
        gross_trade_amt: Option<Amt>,
        net_money: Option<Amt>,
        open_close: Option<OpenClose>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>,
        num_days_interest: Option<FIXInt>,
        accrued_interest_rate: Option<FIXFloat>,
        no_allocs: Option<FIXInt>
    },
    ListCancelRequest {
        list_id: FIXString,
        transact_time: UTCTimestamp,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    ListExecute {
        list_id: FIXString,
        client_bid_id: Option<FIXString>,
        bid_id: Option<FIXString>,
        transact_time: UTCTimestamp,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    ListStatusRequest {
        list_id: FIXString,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    ListStatus {
        list_id: FIXString,
        list_status_type: FIXInt,
        no_rpts: FIXInt,
        list_order_status: FIXInt,
        rpt_seq: FIXInt,
        list_status_text: Option<FIXString>,
        encoded_list_status_text_len: Option<Length>,
        encoded_list_status_text: Option<Data>,
        transact_time: Option<UTCTimestamp>,
        tot_no_orders: FIXInt,
        no_orders: FIXInt
    },
    AllocationInstructionAck {
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        alloc_id: FIXString,
        trade_date: FIXString,
        transact_time: Option<UTCTimestamp>,
        alloc_status: AllocStatus,
        alloc_rej_code: Option<AllocRejCode>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    DontKnowTrade {
        order_id: FIXString,
        exec_id: FIXString,
        dk_reason: DKReason,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        side: Side,
        order_qty: Option<Qty>,
        cash_order_qty: Option<Qty>,
        last_shares: Option<Qty>,
        last_px: Option<Price>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    QuoteRequest {
        quote_req_id: FIXString,
        no_related_sym: FIXInt
    },
    Quote {
        quote_req_id: Option<FIXString>,
        quote_id: FIXString,
        quote_response_level: Option<QuoteResponseLevel>,
        trading_session_id: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        bid_px: Option<Price>,
        offer_px: Option<Price>,
        bid_size: Option<Qty>,
        offer_size: Option<Qty>,
        valid_until_time: Option<UTCTimestamp>,
        bid_spot_rate: Option<Price>,
        offer_spot_rate: Option<Price>,
        bid_forward_points: Option<PriceOffset>,
        offer_forward_points: Option<PriceOffset>,
        transact_time: Option<UTCTimestamp>,
        fut_sett_date: Option<FIXString>,
        ord_type: Option<OrdType>,
        fut_sett_date2: Option<FIXString>,
        order_qty2: Option<Qty>,
        currency: Option<Currency>
    },
    SettlementInstructions {
        settl_inst_id: FIXString,
        settl_inst_trans_type: SettlInstTransType,
        settl_inst_ref_id: FIXString,
        settl_inst_mode: SettlInstMode,
        settl_inst_source: SettlInstSource,
        alloc_account: FIXString,
        settl_location: Option<SettlLocation>,
        trade_date: Option<FIXString>,
        alloc_id: Option<FIXString>,
        last_mkt: Option<FIXString>,
        trading_session_id: Option<FIXString>,
        side: Option<Side>,
        security_type: Option<SecurityType>,
        effective_time: Option<UTCTimestamp>,
        transact_time: UTCTimestamp,
        client_id: Option<FIXString>,
        exec_broker: Option<FIXString>,
        stand_inst_db_type: Option<StandInstDbType>,
        stand_inst_db_name: Option<FIXString>,
        stand_inst_db_id: Option<FIXString>,
        settl_delivery_type: Option<FIXInt>,
        settl_depository_code: Option<FIXString>,
        settl_brkr_code: Option<FIXString>,
        settl_inst_code: Option<FIXString>,
        security_settl_agent_name: Option<FIXString>,
        security_settl_agent_code: Option<FIXString>,
        security_settl_agent_acct_num: Option<FIXString>,
        security_settl_agent_acct_name: Option<FIXString>,
        security_settl_agent_contact_name: Option<FIXString>,
        security_settl_agent_contact_phone: Option<FIXString>,
        cash_settl_agent_name: Option<FIXString>,
        cash_settl_agent_code: Option<FIXString>,
        cash_settl_agent_acct_num: Option<FIXString>,
        cash_settl_agent_acct_name: Option<FIXString>,
        cash_settl_agent_contact_name: Option<FIXString>,
        cash_settl_agent_contact_phone: Option<FIXString>
    },
    MarketDataRequest {
        md_req_id: FIXString,
        subscription_request_type: SubscriptionRequestType,
        market_depth: FIXInt,
        md_update_type: Option<MDUpdateType>,
        aggregated_book: Option<AggregatedBook>,
        no_md_entry_types: FIXInt,
        no_related_sym: FIXInt
    },
    MarketDataSnapshotFullRefresh {
        md_req_id: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        financial_status: Option<FinancialStatus>,
        corporate_action: Option<CorporateAction>,
        total_volume_traded: Option<Qty>,
        no_md_entries: FIXInt
    },
    MarketDataIncrementalRefresh {
        md_req_id: Option<FIXString>,
        no_md_entries: FIXInt
    },
    MarketDataRequestReject {
        md_req_id: FIXString,
        md_req_rej_reason: Option<MDReqRejReason>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    QuoteCancel {
        quote_req_id: Option<FIXString>,
        quote_id: FIXString,
        quote_cancel_type: QuoteCancelType,
        quote_response_level: Option<QuoteResponseLevel>,
        trading_session_id: Option<FIXString>,
        no_quote_entries: FIXInt
    },
    QuoteStatusRequest {
        quote_id: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        side: Option<Side>,
        trading_session_id: Option<FIXString>
    },
    QuoteAcknowledgement {
        quote_req_id: Option<FIXString>,
        quote_id: Option<FIXString>,
        quote_ack_status: QuoteAckStatus,
        quote_reject_reason: Option<QuoteRejectReason>,
        quote_response_level: Option<QuoteResponseLevel>,
        trading_session_id: Option<FIXString>,
        text: Option<FIXString>,
        no_quote_sets: Option<FIXInt>
    },
    SecurityDefinitionRequest {
        security_req_id: FIXString,
        security_request_type: SecurityRequestType,
        symbol: Option<FIXString>,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        currency: Option<Currency>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>,
        trading_session_id: Option<FIXString>,
        no_related_sym: Option<FIXInt>
    },
    SecurityDefinition {
        security_req_id: FIXString,
        security_response_id: FIXString,
        security_response_type: Option<SecurityResponseType>,
        total_num_securities: FIXInt,
        symbol: Option<FIXString>,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        currency: Option<Currency>,
        trading_session_id: Option<FIXString>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>,
        no_related_sym: Option<FIXInt>
    },
    SecurityStatusRequest {
        security_status_req_id: FIXString,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        currency: Option<Currency>,
        subscription_request_type: SubscriptionRequestType,
        trading_session_id: Option<FIXString>
    },
    SecurityStatus {
        security_status_req_id: Option<FIXString>,
        symbol: FIXString,
        symbol_sfx: Option<FIXString>,
        security_id: Option<FIXString>,
        id_source: Option<IDSource>,
        security_type: Option<SecurityType>,
        maturity_month_year: Option<MonthYear>,
        maturity_day: Option<DayOfMonth>,
        put_or_call: Option<PutOrCall>,
        strike_price: Option<Price>,
        opt_attribute: Option<FIXChar>,
        contract_multiplier: Option<FIXFloat>,
        coupon_rate: Option<FIXFloat>,
        security_exchange: Option<FIXString>,
        issuer: Option<FIXString>,
        encoded_issuer_len: Option<Length>,
        encoded_issuer: Option<Data>,
        security_desc: Option<FIXString>,
        encoded_security_desc_len: Option<Length>,
        encoded_security_desc: Option<Data>,
        currency: Option<Currency>,
        trading_session_id: Option<FIXString>,
        unsolicited_indicator: Option<UnsolicitedIndicator>,
        security_trading_status: Option<SecurityTradingStatus>,
        financial_status: Option<FinancialStatus>,
        corporate_action: Option<CorporateAction>,
        halt_reason_char: Option<HaltReasonChar>,
        in_view_of_common: Option<InViewOfCommon>,
        due_to_related: Option<DueToRelated>,
        buy_volume: Option<Qty>,
        sell_volume: Option<Qty>,
        high_px: Option<Price>,
        low_px: Option<Price>,
        last_px: Option<Price>,
        transact_time: Option<UTCTimestamp>,
        adjustment: Option<Adjustment>
    },
    TradingSessionStatusRequest {
        trad_ses_req_id: FIXString,
        trading_session_id: Option<FIXString>,
        trad_ses_method: Option<TradSesMethod>,
        trad_ses_mode: Option<TradSesMode>,
        subscription_request_type: SubscriptionRequestType
    },
    TradingSessionStatus {
        trad_ses_req_id: Option<FIXString>,
        trading_session_id: FIXString,
        trad_ses_method: Option<TradSesMethod>,
        trad_ses_mode: Option<TradSesMode>,
        unsolicited_indicator: Option<UnsolicitedIndicator>,
        trad_ses_status: TradSesStatus,
        trad_ses_start_time: Option<UTCTimestamp>,
        trad_ses_open_time: Option<UTCTimestamp>,
        trad_ses_pre_close_time: Option<UTCTimestamp>,
        trad_ses_close_time: Option<UTCTimestamp>,
        trad_ses_end_time: Option<UTCTimestamp>,
        total_volume_traded: Option<Qty>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    MassQuote {
        quote_req_id: Option<FIXString>,
        quote_id: FIXString,
        quote_response_level: Option<QuoteResponseLevel>,
        def_bid_size: Option<Qty>,
        def_offer_size: Option<Qty>,
        no_quote_sets: FIXInt
    },
    BusinessMessageReject {
        ref_seq_num: Option<FIXInt>,
        ref_msg_type: FIXString,
        business_reject_ref_id: Option<FIXString>,
        business_reject_reason: BusinessRejectReason,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    BidRequest {
        bid_id: Option<FIXString>,
        client_bid_id: FIXString,
        bid_request_trans_type: BidRequestTransType,
        list_name: Option<FIXString>,
        total_num_securities: FIXInt,
        bid_type: FIXInt,
        num_tickets: Option<FIXInt>,
        currency: Option<Currency>,
        side_value1: Option<Amt>,
        side_value2: Option<Amt>,
        no_bid_descriptors: Option<FIXInt>,
        no_bid_components: Option<FIXInt>,
        liquidity_ind_type: Option<LiquidityIndType>,
        wt_average_liquidity: Option<FIXFloat>,
        exchange_for_physical: Option<ExchangeForPhysical>,
        out_main_cntry_u_index: Option<Amt>,
        cross_percent: Option<FIXFloat>,
        prog_rpt_reqs: Option<ProgRptReqs>,
        prog_period_interval: Option<FIXInt>,
        inc_tax_ind: Option<IncTaxInd>,
        forex_req: Option<ForexReq>,
        num_bidders: Option<FIXInt>,
        trade_date: Option<FIXString>,
        trade_type: TradeType,
        basis_px_type: BasisPxType,
        strike_time: Option<UTCTimestamp>,
        text: Option<FIXString>,
        encoded_text_len: Option<Length>,
        encoded_text: Option<Data>
    },
    BidResponse {
        bid_id: Option<FIXString>,
        client_bid_id: Option<FIXString>,
        no_bid_components: FIXInt
    },
    ListStrikePrice {
        list_id: FIXString,
        tot_no_strikes: FIXInt,
        no_strikes: FIXInt
    }
}