// use std::collections::HashSet;
// use std::fmt::Write;
// use std::time::Duration;

// use anyhow::Result;
// use http::header::AUTHORIZATION;
// use http::header::CONTENT_TYPE;
// use http::header::DATE;
// use http::HeaderValue;
// use log::debug;
// use once_cell::sync::Lazy;
// use percent_encoding::utf8_percent_encode;

// use super::credential::Credential;
// use crate::ctx::SigningContext;
// use crate::ctx::SigningMethod;
// use crate::hash::base64_hmac_sha1;
// use crate::request::SignableRequest;
// use crate::time;
// use crate::time::format_http_date;
// use crate::time::DateTime;

// const CONTENT_MD5: &str = "content-md5";
// const ACCEPT_JSON: &str = "application/json";

// pub struct Signer {
//     scheme: String,
//     endpoint: String,

//     // resource_uri_parameters: String,
// }

// impl Signer {
//     pub fn new(endpoint: &str) -> Self {
//         Self {
//             scheme: "https".to_owned(),
//             endpoint: endpoint.to_owned()
//         }
//     }

//     fn build(&self, req: &mut impl SignableRequest, cred: &Credential) -> Result<SigningContext> {
//         let now = time::now();
//         let mut ctx = req.build();

//         let string_to_sign = string_to_sign(ctx, cred, now, method, bucket);


//     }

    
// }

// /// Construct string to sign.
// ///
// /// # Format
// ///
// /// ```text
// ///   HTTPMethod + "\n"
// /// + Accept + "\n"              
// /// + ContentMD5 + "\n"
// /// + ContentType + "\n"
// /// + Date + "\n"
// /// + CanonicalizedHeaders
// /// + CanonicalizedResource
// /// ```
// fn string_to_sign(
//     ctx: &mut SigningContext,
//     cred: &Credential,
//     now: DateTime,
//     method: SigningMethod,
//     bucket: &str,
// ) -> Result<String> {
//     let mut s = String::new();
//     s.write_str(ctx.method.as_str())?;
//     s.write_str("\n")?;
//     s.write_str(ACCEPT_JSON)?;
//     s.write_str("\n")?;
//     s.write_str(ctx.header_get_or_default(&CONTENT_MD5.parse()?)?)?;
//     s.write_str("\n")?;
//     s.write_str(ctx.header_get_or_default(&CONTENT_TYPE)?)?;
//     s.write_str("\n")?;
//     match method {
//         SigningMethod::Header => {
//             writeln!(&mut s, "{}", format_http_date(now))?;
//         }
//         SigningMethod::Query(expires) => {
//             writeln!(
//                 &mut s,
//                 "{}",
//                 (now + chrono::Duration::from_std(expires).unwrap()).timestamp()
//             )?;
//         }
//     }

//     {
//         let headers = canonicalize_header(ctx, method, cred)?;
//         if !headers.is_empty() {
//             writeln!(&mut s, "{headers}",)?;
//         }
//     }
//     write!(
//         &mut s,
//         "{}",
//         canonicalize_resource(ctx, bucket, method, cred)
//     )?;

//     debug!("string to sign: {}", &s);
//     Ok(s)
// }

// /// Build canonicalize header
// ///
// /// # Reference
// ///
// /// [Building CanonicalizedOSSHeaders](https://help.aliyun.com/document_detail/31951.html#section-w2k-sw2-xdb)
// fn canonicalize_header(
//     ctx: &mut SigningContext,
//     method: SigningMethod,
//     cred: &Credential,
// ) -> Result<String> {
//     if method == SigningMethod::Header {
//         // Insert security token
//         if let Some(token) = &cred.security_token {
//             ctx.headers.insert("x-oss-security-token", token.parse()?);
//         }
//     }

//     Ok(SigningContext::header_to_string(
//         ctx.header_to_vec_with_prefix("x-acs-"),
//         ":",
//         "\n",
//     ))
// }

// /// Build canonicalize resource
// ///
// /// # Reference
// ///
// /// [Building CanonicalizedResource](https://help.aliyun.com/document_detail/31951.html#section-w2k-sw2-xdb)
// fn canonicalize_resource(
//     ctx: &mut SigningContext,
//     bucket: &str,
//     method: SigningMethod,
//     cred: &Credential,
// ) -> String {
//     if let SigningMethod::Query(_) = method {
//         // Insert security token
//         if let Some(token) = &cred.security_token {
//             ctx.query.push((
//                 "security-token".to_string(),
//                 utf8_percent_encode(token, percent_encoding::NON_ALPHANUMERIC).to_string(),
//             ));
//         };
//     }

//     let params = ctx.query_to_vec_with_filter(is_sub_resource);

//     // OSS requires that the query string be percent-decoded.
//     let params_str = SigningContext::query_to_percent_decoded_string(params, "=", "&");

//     if params_str.is_empty() {
//         format!("/{bucket}{}", ctx.path_percent_decoded())
//     } else {
//         format!("/{bucket}{}?{params_str}", ctx.path_percent_decoded())
//     }
// }

// fn is_sub_resource(v: &str) -> bool {
//     SUB_RESOURCES.contains(&v)
// }

// /// This list is copied from <https://github.com/aliyun/aliyun-oss-go-sdk/blob/master/oss/conn.go>
// static SUB_RESOURCES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
//     HashSet::from([
//         "acl",
//         "uploads",
//         "location",
//         "cors",
//         "logging",
//         "website",
//         "referer",
//         "lifecycle",
//         "delete",
//         "append",
//         "tagging",
//         "objectMeta",
//         "uploadId",
//         "partNumber",
//         "security-token",
//         "position",
//         "img",
//         "style",
//         "styleName",
//         "replication",
//         "replicationProgress",
//         "replicationLocation",
//         "cname",
//         "bucketInfo",
//         "comp",
//         "qos",
//         "live",
//         "status",
//         "vod",
//         "startTime",
//         "endTime",
//         "symlink",
//         "x-oss-process",
//         "response-content-type",
//         "x-oss-traffic-limit",
//         "response-content-language",
//         "response-expires",
//         "response-cache-control",
//         "response-content-disposition",
//         "response-content-encoding",
//         "udf",
//         "udfName",
//         "udfImage",
//         "udfId",
//         "udfImageDesc",
//         "udfApplication",
//         "comp",
//         "udfApplicationLog",
//         "restore",
//         "callback",
//         "callback-var",
//         "qosInfo",
//         "policy",
//         "stat",
//         "encryption",
//         "versions",
//         "versioning",
//         "versionId",
//         "requestPayment",
//         "x-oss-request-payer",
//         "sequential",
//         "inventory",
//         "inventoryId",
//         "continuation-token",
//         "asyncFetch",
//         "worm",
//         "wormId",
//         "wormExtend",
//         "withHashContext",
//         "x-oss-enable-md5",
//         "x-oss-enable-sha1",
//         "x-oss-enable-sha256",
//         "x-oss-hash-ctx",
//         "x-oss-md5-ctx",
//         "transferAcceleration",
//         "regionList",
//         "cloudboxes",
//         "x-oss-ac-source-ip",
//         "x-oss-ac-subnet-mask",
//         "x-oss-ac-vpc-id",
//         "x-oss-ac-forward-allow",
//         "metaQuery",
//     ])
// });
