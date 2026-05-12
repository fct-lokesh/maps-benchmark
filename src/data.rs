use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallLog {
    pub protocol: String,
    pub action: String,

    pub srcip: String,
    pub srcport: String,

    pub dstip: String,
    pub dstport: String,

    pub status: String,

    pub url: String,

    pub device_model: String,

    #[serde(rename = "device_serial_Id")]
    pub device_serial_id: String,

    pub log_id: String,
    pub log_type: String,
    pub log_component: String,
    pub log_version: String,

    pub severity: String,

    pub firewall_rule_id: String,
    pub firewall_rule_name: String,
    pub firewall_rule_section: String,

    pub web_policy_id: String,

    pub http_category: String,
    pub http_category_type: String,

    pub category_type: String,
    pub category: String,

    pub bytes_sent: String,
    pub bytes_received: String,

    pub http_status: String,

    pub app_is_cloud: String,

    pub used_quota: String,

    pub src_zone_type: String,
    pub src_zone: String,

    pub dst_zone_type: String,
    pub dst_zone: String,

    pub src_country: String,
    pub dst_country: String,

    pub domain: String,

    pub exceptions: String,

    pub rule_id: String,
    pub rule_name: String,

    pub con_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallLogExpanded {
    // Original Fields
    pub protocol: String,
    pub action: String,

    pub srcip: String,
    pub srcport: String,

    pub dstip: String,
    pub dstport: String,

    pub status: String,

    pub url: String,

    pub device_model: String,

    #[serde(rename = "device_serial_Id")]
    pub device_serial_id: String,

    pub log_id: String,
    pub log_type: String,
    pub log_component: String,
    pub log_version: String,

    pub severity: String,

    pub firewall_rule_id: String,
    pub firewall_rule_name: String,
    pub firewall_rule_section: String,

    pub web_policy_id: String,

    pub http_category: String,
    pub http_category_type: String,

    pub category_type: String,
    pub category: String,

    pub bytes_sent: String,
    pub bytes_received: String,

    pub http_status: String,

    pub app_is_cloud: String,

    pub used_quota: String,

    pub src_zone_type: String,
    pub src_zone: String,

    pub dst_zone_type: String,
    pub dst_zone: String,

    pub src_country: String,
    pub dst_country: String,

    pub domain: String,

    pub exceptions: String,

    pub rule_id: String,
    pub rule_name: String,

    pub con_id: String,

    // Duplicate / Expanded Fields
    pub protocol_upper: String,
    pub action_upper: String,

    pub source_ip: String,
    pub source_port: String,

    pub destination_ip: String,
    pub destination_port: String,

    pub connection_status: String,

    pub request_url: String,

    pub firewall_device_model: String,

    pub firewall_device_serial_id: String,

    pub firewall_log_id: String,
    pub firewall_log_type: String,
    pub firewall_log_component: String,
    pub firewall_log_version: String,

    pub log_severity: String,

    pub fw_rule_id: String,
    pub fw_rule_name: String,
    pub fw_rule_section: String,

    pub web_filter_policy_id: String,

    pub web_http_category: String,
    pub web_http_category_type: String,

    pub web_category_type: String,
    pub web_category: String,

    pub tx_bytes: String,
    pub rx_bytes: String,

    pub http_response_status: String,

    pub cloud_application: String,

    pub quota_used: String,

    pub source_zone_type: String,
    pub source_zone: String,

    pub destination_zone_type: String,
    pub destination_zone: String,

    pub source_country: String,
    pub destination_country: String,

    pub request_domain: String,

    pub firewall_exceptions: String,

    pub network_rule_id: String,
    pub network_rule_name: String,

    pub connection_id: String,
}
impl FirewallLogExpanded {
    #[inline]
    pub fn to_kv_vec(self) -> Vec<(String, String)> {
        vec![
            // Original
            ("protocol".into(), self.protocol),
            ("action".into(), self.action),
            ("srcip".into(), self.srcip),
            ("srcport".into(), self.srcport),
            ("dstip".into(), self.dstip),
            ("dstport".into(), self.dstport),
            ("status".into(), self.status),
            ("url".into(), self.url),
            ("device_model".into(), self.device_model),
            ("device_serial_Id".into(), self.device_serial_id),
            ("log_id".into(), self.log_id),
            ("log_type".into(), self.log_type),
            ("log_component".into(), self.log_component),
            ("log_version".into(), self.log_version),
            ("severity".into(), self.severity),
            ("firewall_rule_id".into(), self.firewall_rule_id),
            ("firewall_rule_name".into(), self.firewall_rule_name),
            ("firewall_rule_section".into(), self.firewall_rule_section),
            ("web_policy_id".into(), self.web_policy_id),
            ("http_category".into(), self.http_category),
            ("http_category_type".into(), self.http_category_type),
            ("category_type".into(), self.category_type),
            ("category".into(), self.category),
            ("bytes_sent".into(), self.bytes_sent),
            ("bytes_received".into(), self.bytes_received),
            ("http_status".into(), self.http_status),
            ("app_is_cloud".into(), self.app_is_cloud),
            ("used_quota".into(), self.used_quota),
            ("src_zone_type".into(), self.src_zone_type),
            ("src_zone".into(), self.src_zone),
            ("dst_zone_type".into(), self.dst_zone_type),
            ("dst_zone".into(), self.dst_zone),
            ("src_country".into(), self.src_country),
            ("dst_country".into(), self.dst_country),
            ("domain".into(), self.domain),
            ("exceptions".into(), self.exceptions),
            ("rule_id".into(), self.rule_id),
            ("rule_name".into(), self.rule_name),
            ("con_id".into(), self.con_id),
            // Expanded
            ("protocol_upper".into(), self.protocol_upper),
            ("action_upper".into(), self.action_upper),
            ("source_ip".into(), self.source_ip),
            ("source_port".into(), self.source_port),
            ("destination_ip".into(), self.destination_ip),
            ("destination_port".into(), self.destination_port),
            ("connection_status".into(), self.connection_status),
            ("request_url".into(), self.request_url),
            ("firewall_device_model".into(), self.firewall_device_model),
            (
                "firewall_device_serial_id".into(),
                self.firewall_device_serial_id,
            ),
            ("firewall_log_id".into(), self.firewall_log_id),
            ("firewall_log_type".into(), self.firewall_log_type),
            ("firewall_log_component".into(), self.firewall_log_component),
            ("firewall_log_version".into(), self.firewall_log_version),
            ("log_severity".into(), self.log_severity),
            ("fw_rule_id".into(), self.fw_rule_id),
            ("fw_rule_name".into(), self.fw_rule_name),
            ("fw_rule_section".into(), self.fw_rule_section),
            ("web_filter_policy_id".into(), self.web_filter_policy_id),
            ("web_http_category".into(), self.web_http_category),
            ("web_http_category_type".into(), self.web_http_category_type),
            ("web_category_type".into(), self.web_category_type),
            ("web_category".into(), self.web_category),
            ("tx_bytes".into(), self.tx_bytes),
            ("rx_bytes".into(), self.rx_bytes),
            ("http_response_status".into(), self.http_response_status),
            ("cloud_application".into(), self.cloud_application),
            ("quota_used".into(), self.quota_used),
            ("source_zone_type".into(), self.source_zone_type),
            ("source_zone".into(), self.source_zone),
            ("destination_zone_type".into(), self.destination_zone_type),
            ("destination_zone".into(), self.destination_zone),
            ("source_country".into(), self.source_country),
            ("destination_country".into(), self.destination_country),
            ("request_domain".into(), self.request_domain),
            ("firewall_exceptions".into(), self.firewall_exceptions),
            ("network_rule_id".into(), self.network_rule_id),
            ("network_rule_name".into(), self.network_rule_name),
            ("connection_id".into(), self.connection_id),
        ]
    }
}
impl FirewallLog {
    #[inline]
    pub fn to_kv_vec(self) -> Vec<(String, String)> {
        vec![
            ("protocol".into(), self.protocol),
            ("action".into(), self.action),
            ("srcip".into(), self.srcip),
            ("srcport".into(), self.srcport),
            ("dstip".into(), self.dstip),
            ("dstport".into(), self.dstport),
            ("status".into(), self.status),
            ("url".into(), self.url),
            ("device_model".into(), self.device_model),
            ("device_serial_Id".into(), self.device_serial_id),
            ("log_id".into(), self.log_id),
            ("log_type".into(), self.log_type),
            ("log_component".into(), self.log_component),
            ("log_version".into(), self.log_version),
            ("severity".into(), self.severity),
            ("firewall_rule_id".into(), self.firewall_rule_id),
            ("firewall_rule_name".into(), self.firewall_rule_name),
            ("firewall_rule_section".into(), self.firewall_rule_section),
            ("web_policy_id".into(), self.web_policy_id),
            ("http_category".into(), self.http_category),
            ("http_category_type".into(), self.http_category_type),
            ("category_type".into(), self.category_type),
            ("category".into(), self.category),
            ("bytes_sent".into(), self.bytes_sent),
            ("bytes_received".into(), self.bytes_received),
            ("http_status".into(), self.http_status),
            ("app_is_cloud".into(), self.app_is_cloud),
            ("used_quota".into(), self.used_quota),
            ("src_zone_type".into(), self.src_zone_type),
            ("src_zone".into(), self.src_zone),
            ("dst_zone_type".into(), self.dst_zone_type),
            ("dst_zone".into(), self.dst_zone),
            ("src_country".into(), self.src_country),
            ("dst_country".into(), self.dst_country),
            ("domain".into(), self.domain),
            ("exceptions".into(), self.exceptions),
            ("rule_id".into(), self.rule_id),
            ("rule_name".into(), self.rule_name),
            ("con_id".into(), self.con_id),
        ]
    }
}
impl From<FirewallLog> for FirewallLogExpanded {
    fn from(v: FirewallLog) -> Self {
        Self {
            // Original
            protocol: v.protocol.clone(),
            action: v.action.clone(),

            srcip: v.srcip.clone(),
            srcport: v.srcport.clone(),

            dstip: v.dstip.clone(),
            dstport: v.dstport.clone(),

            status: v.status.clone(),

            url: v.url.clone(),

            device_model: v.device_model.clone(),

            device_serial_id: v.device_serial_id.clone(),

            log_id: v.log_id.clone(),
            log_type: v.log_type.clone(),
            log_component: v.log_component.clone(),
            log_version: v.log_version.clone(),

            severity: v.severity.clone(),

            firewall_rule_id: v.firewall_rule_id.clone(),
            firewall_rule_name: v.firewall_rule_name.clone(),
            firewall_rule_section: v.firewall_rule_section.clone(),

            web_policy_id: v.web_policy_id.clone(),

            http_category: v.http_category.clone(),
            http_category_type: v.http_category_type.clone(),

            category_type: v.category_type.clone(),
            category: v.category.clone(),

            bytes_sent: v.bytes_sent.clone(),
            bytes_received: v.bytes_received.clone(),

            http_status: v.http_status.clone(),

            app_is_cloud: v.app_is_cloud.clone(),

            used_quota: v.used_quota.clone(),

            src_zone_type: v.src_zone_type.clone(),
            src_zone: v.src_zone.clone(),

            dst_zone_type: v.dst_zone_type.clone(),
            dst_zone: v.dst_zone.clone(),

            src_country: v.src_country.clone(),
            dst_country: v.dst_country.clone(),

            domain: v.domain.clone(),

            exceptions: v.exceptions.clone(),

            rule_id: v.rule_id.clone(),
            rule_name: v.rule_name.clone(),

            con_id: v.con_id.clone(),

            // Expanded
            protocol_upper: v.protocol.to_uppercase(),
            action_upper: v.action.to_uppercase(),

            source_ip: v.srcip,
            source_port: v.srcport,

            destination_ip: v.dstip,
            destination_port: v.dstport,

            connection_status: v.status,

            request_url: v.url,

            firewall_device_model: v.device_model,

            firewall_device_serial_id: v.device_serial_id,

            firewall_log_id: v.log_id,
            firewall_log_type: v.log_type,
            firewall_log_component: v.log_component,
            firewall_log_version: v.log_version,

            log_severity: v.severity,

            fw_rule_id: v.firewall_rule_id,
            fw_rule_name: v.firewall_rule_name,
            fw_rule_section: v.firewall_rule_section,

            web_filter_policy_id: v.web_policy_id,

            web_http_category: v.http_category,
            web_http_category_type: v.http_category_type,

            web_category_type: v.category_type,
            web_category: v.category,

            tx_bytes: v.bytes_sent,
            rx_bytes: v.bytes_received,

            http_response_status: v.http_status,

            cloud_application: v.app_is_cloud,

            quota_used: v.used_quota,

            source_zone_type: v.src_zone_type,
            source_zone: v.src_zone,

            destination_zone_type: v.dst_zone_type,
            destination_zone: v.dst_zone,

            source_country: v.src_country,
            destination_country: v.dst_country,

            request_domain: v.domain,

            firewall_exceptions: v.exceptions,

            network_rule_id: v.rule_id,
            network_rule_name: v.rule_name,

            connection_id: v.con_id,
        }
    }
}

pub fn sample_data_fml() -> Vec<FirewallLogExpanded> {
    sample_data_fm().into_iter().map(|d| d.into()).collect()
}
pub fn sample_data_fm() -> Vec<FirewallLog> {
    let firewall_logs = vec![
        FirewallLog {
            protocol: "PROTO_SSL_001".into(),
            action: "Do not decrypt".into(),

            srcip: "192.134.6.24".into(),
            srcport: "52166".into(),

            dstip: "72.154.7.107".into(),
            dstport: "443".into(),

            status: "STATUS_SSL_001".into(),

            url: "URL_SSL_001".into(),

            device_model: "XGS128".into(),

            device_serial_id: "X12508CHPMM2Q01".into(),

            log_id: "058534619004".into(),
            log_type: "Content Filtering".into(),
            log_component: "SSL".into(),
            log_version: "1".into(),

            severity: "Information".into(),

            firewall_rule_id: "FW_RULE_SSL_001".into(),
            firewall_rule_name: "FW_RULE_NAME_SSL_001".into(),
            firewall_rule_section: "FW_RULE_SECTION_SSL_001".into(),

            web_policy_id: "WEB_POLICY_SSL_001".into(),

            http_category: "HTTP_CAT_SSL_001".into(),
            http_category_type: "HTTP_CAT_TYPE_SSL_001".into(),

            category_type: "CATEGORY_TYPE_SSL_001".into(),
            category: "Information Technology".into(),

            bytes_sent: "BYTES_SENT_SSL_001".into(),
            bytes_received: "BYTES_RECEIVED_SSL_001".into(),

            http_status: "HTTP_STATUS_SSL_001".into(),

            app_is_cloud: "APP_IS_CLOUD_SSL_001".into(),

            used_quota: "USED_QUOTA_SSL_001".into(),

            src_zone_type: "LAN".into(),
            src_zone: "LAN".into(),

            dst_zone_type: "WAN".into(),
            dst_zone: "WAN".into(),

            src_country: "FRA".into(),
            dst_country: "USA".into(),

            domain: "DOMAIN_SSL_001".into(),

            exceptions: "av,https,validation,policy,zero-day protection".into(),

            rule_id: "1".into(),
            rule_name: "Exclusions by website or category".into(),

            con_id: "3351685632".into(),
        },
        FirewallLog {
            protocol: "TCP".into(),
            action: "Allowed".into(),

            srcip: "192.134.6.76".into(),
            srcport: "59682".into(),

            dstip: "72.153.5.137".into(),
            dstport: "443".into(),

            status: "0".into(),

            url: "https://array516.prod.do.dsp.mp.microsoft.com".into(),

            device_model: "XGS128".into(),

            device_serial_id: "X12508CHPMM2Q01".into(),

            log_id: "050901616001".into(),
            log_type: "Content Filtering".into(),
            log_component: "HTTP".into(),
            log_version: "1".into(),

            severity: "Information".into(),

            firewall_rule_id: "5".into(),
            firewall_rule_name: "#Default_Network_Policy".into(),
            firewall_rule_section: "Local rule".into(),

            web_policy_id: "1".into(),

            http_category: "Information Technology".into(),
            http_category_type: "Acceptable".into(),

            category_type: "Acceptable".into(),
            category: "Information Technology".into(),

            bytes_sent: "1959".into(),
            bytes_received: "3472".into(),

            http_status: "0".into(),

            app_is_cloud: "FALSE".into(),

            used_quota: "0".into(),

            src_zone_type: "LAN".into(),
            src_zone: "LAN".into(),

            dst_zone_type: "WAN".into(),
            dst_zone: "WAN".into(),

            src_country: "FRA".into(),
            dst_country: "USA".into(),

            domain: "array516.prod.do.dsp.mp.microsoft.com".into(),

            exceptions: "av,https,validation,policy,zero-day protection".into(),

            rule_id: "5".into(),
            rule_name: "#Default_Network_Policy".into(),

            con_id: "1681803136".into(),
        },
        FirewallLog {
            protocol: "PROTO_IPSEC_003".into(),
            action: "System".into(),

            srcip: "103.50.152.74".into(),
            srcport: "SRCPORT_IPSEC_003".into(),

            dstip: "144.24.98.191".into(),
            dstport: "DSTPORT_IPSEC_003".into(),

            status: "Failed".into(),

            url: "URL_IPSEC_003".into(),

            device_model: "XGS128".into(),

            device_serial_id: "X12508CHPMM2Q01".into(),

            log_id: "062511418055".into(),
            log_type: "Event".into(),
            log_component: "IPSec".into(),
            log_version: "1".into(),

            severity: "Warning".into(),

            firewall_rule_id: "FW_RULE_IPSEC_003".into(),
            firewall_rule_name: "FW_RULE_NAME_IPSEC_003".into(),
            firewall_rule_section: "FW_RULE_SECTION_IPSEC_003".into(),

            web_policy_id: "WEB_POLICY_IPSEC_003".into(),

            http_category: "HTTP_CAT_IPSEC_003".into(),
            http_category_type: "HTTP_CAT_TYPE_IPSEC_003".into(),

            category_type: "CATEGORY_TYPE_IPSEC_003".into(),
            category: "CATEGORY_IPSEC_003".into(),

            bytes_sent: "BYTES_SENT_IPSEC_003".into(),
            bytes_received: "BYTES_RECEIVED_IPSEC_003".into(),

            http_status: "HTTP_STATUS_IPSEC_003".into(),

            app_is_cloud: "APP_IS_CLOUD_IPSEC_003".into(),

            used_quota: "USED_QUOTA_IPSEC_003".into(),

            src_zone_type: "SRC_ZONE_TYPE_IPSEC_003".into(),
            src_zone: "SRC_ZONE_IPSEC_003".into(),

            dst_zone_type: "DST_ZONE_TYPE_IPSEC_003".into(),
            dst_zone: "DST_ZONE_IPSEC_003".into(),

            src_country: "IND".into(),
            dst_country: "IND".into(),

            domain: "DOMAIN_IPSEC_003".into(),

            exceptions: "EXCEPTIONS_IPSEC_003".into(),

            rule_id: "RULE_ID_IPSEC_003".into(),
            rule_name: "RULE_NAME_IPSEC_003".into(),

            con_id: "CON_ID_IPSEC_003".into(),
        },
    ];
    firewall_logs
}

pub fn sample_data() -> Vec<Vec<(String, String)>> {
    let logs  = vec![
        vec![
            ("action".into(), "Do not decrypt".into()),
            ("srcip".into(), "192.134.6.24".into()),
            ("srcport".into(), "52166".into()),
            ("dstip".into(), "72.154.7.107".into()),
            ("dstport".into(), "443".into()),
            ("device_model".into(), "XGS128".into()),
            ("device_serial_Id".into(), "X12508CHPMM2Q01".into()),
            ("log_id".into(), "058534619004".into()),
            ("log_type".into(), "Content Filtering".into()),
            ("log_component".into(), "SSL".into()),
            ("log_version".into(), "1".into()),
            ("severity".into(), "Information".into()),
            ("category".into(), "Information Technology".into()),
            ("filename".into(), "Maximum compatibility".into()),
            ("src_zone_type".into(), "LAN".into()),
            ("src_zone".into(), "LAN".into()),
            ("dst_zone_type".into(), "WAN".into()),
            ("dst_zone".into(), "WAN".into()),
            ("src_country".into(), "FRA".into()),
            ("dst_country".into(), "USA".into()),
            ("exceptions".into(), "av,https,validation,policy,zero-day protection".into()),
            ("rule_id".into(), "1".into()),
            ("rule_name".into(), "Exclusions by website or category".into()),
            ("profile_id".into(), "1".into()),
            ("profile_name".into(), "Maximum".into()),
            ("bitmask".into(), "Valid".into()),
            ("key_type".into(), "KEY_TYPE__EC".into()),
            ("key_param".into(), "EC secp256r1".into()),
            ("fingerprint".into(), "93:18:aa:cf:92:43:1d:de:9f:5b:93:82:f2:f0:b4:6c:ea:25:ef:6e".into()),
            ("cert_chain_served".into(), "TRUE".into()),
            ("cipher_suite".into(), "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384".into()),
            ("tls_version".into(), "TLS1.2".into()),
            ("sni".into(), "array814.prod.do.dsp.mp.microsoft.com".into()),
            ("con_id".into(), "3351685632".into()),
        ],

        vec![
            ("protocol".into(), "TCP".into()),
            ("action".into(), "Allowed".into()),
            ("srcip".into(), "192.134.6.76".into()),
            ("srcport".into(), "59682".into()),
            ("dstip".into(), "72.153.5.137".into()),
            ("dstport".into(), "443".into()),
            ("status".into(), "0".into()),
            ("url".into(), "https://array516.prod.do.dsp.mp.microsoft.com".into()),
            ("device_model".into(), "XGS128".into()),
            ("device_serial_Id".into(), "X12508CHPMM2Q01".into()),
            ("log_id".into(), "050901616001".into()),
            ("log_type".into(), "Content Filtering".into()),
            ("log_component".into(), "HTTP".into()),
            ("log_version".into(), "1".into()),
            ("severity".into(), "Information".into()),
            ("firewall_rule_id".into(), "5".into()),
            ("firewall_rule_name".into(), "#Default_Network_Policy".into()),
            ("firewall_rule_section".into(), "Local rule".into()),
            ("web_policy_id".into(), "1".into()),
            ("http_category".into(), "Information Technology".into()),
            ("http_category_type".into(), "Acceptable".into()),
            ("category_type".into(), "Acceptable".into()),
            ("category".into(), "Information Technology".into()),
            ("bytes_sent".into(), "1959".into()),
            ("bytes_received".into(), "3472".into()),
            ("http_status".into(), "0".into()),
            ("app_is_cloud".into(), "FALSE".into()),
            ("used_quota".into(), "0".into()),
            ("src_zone_type".into(), "LAN".into()),
            ("src_zone".into(), "LAN".into()),
            ("dst_zone_type".into(), "WAN".into()),
            ("dst_zone".into(), "WAN".into()),
            ("src_country".into(), "FRA".into()),
            ("dst_country".into(), "USA".into()),
            ("domain".into(), "array516.prod.do.dsp.mp.microsoft.com".into()),
            ("exceptions".into(), "av,https,validation,policy,zero-day protection".into()),
            ("rule_id".into(), "5".into()),
            ("rule_name".into(), "#Default_Network_Policy".into()),
            ("con_id".into(), "1681803136".into()),
        ],

        vec![
            ("action".into(), "System".into()),
            ("srcip".into(), "103.50.152.74".into()),
            ("dstip".into(), "144.24.98.191".into()),
            ("status".into(), "Failed".into()),
            ("device_model".into(), "XGS128".into()),
            ("device_serial_Id".into(), "X12508CHPMM2Q01".into()),
            ("log_id".into(), "062511418055".into()),
            ("log_type".into(), "Event".into()),
            ("log_component".into(), "IPSec".into()),
            ("log_version".into(), "1".into()),
            ("severity".into(), "Warning".into()),
            ("src_country".into(), "IND".into()),
            ("dst_country".into(), "IND".into()),
            (
                "message".into(),
                "M2903yBSS_HA_ACT_BhgydyBank-1 - IKE message (18015BA0) retransmission to 144.24.98.191 timed out. Check if the remote gateway is reachable. (Remote: 144.24.98.191)".into()
            ),
            ("con_name".into(), "M2903yBSS_HA_ACT_BhgydyBank-1".into()),
        ],
    ];
    logs
}
