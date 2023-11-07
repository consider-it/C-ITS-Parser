#![cfg(not(doctest))]
#![allow(unused, non_upper_case_globals)]
extern crate alloc;

use rasn::prelude::*;
// =====================================================
// Ieee1609Dot2
// { iso(1) identified-organization(3) ieee(111) standards-association-numbered-series-standards(2) wave-stds(1609) dot2(2) base(1) schema(1) major-version-2(2) minor-version-6(6) }
// =====================================================

///*
/// * @brief This type is defined only for backwards compatibility.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Aes128CcmCiphertext(pub One28BitCcmCiphertext);

///*
/// * @brief This structure contains an individual AppExtension. AppExtensions
/// * specified in this standard are drawn from the ASN.1 Information Object Set
/// * SetCertExtensions. This set, and its use in the AppExtension type, is
/// * structured so that each AppExtension is associated with a
/// * CertIssueExtension and a CertRequestExtension and all are identified by
/// * the same id value. In this structure:
/// *
/// * @param id: identifies the extension type.
/// *
/// * @param content: provides the content of the extension.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AppExtension {
    pub id: ExtId,
    pub content: Any,
}

impl AppExtension {
    pub fn new(id: ExtId, content: Any) -> Self {
        Self { id, content }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum CertIssueExtensionPermissions {
    Specific(Any),
    All(()),
}

///*
/// * @brief This field contains an individual CertIssueExtension.
/// * CertIssueExtensions specified in this standard are drawn from the ASN.1
/// * Information Object Set SetCertExtensions. This set, and its use in the
/// * CertIssueExtension type, is structured so that each CertIssueExtension
/// * is associated with a AppExtension and a CertRequestExtension and all are
/// * identified by the same id value. In this structure:
/// *
/// * @param id: identifies the extension type.
/// *
/// * @param permissions: indicates the permissions. Within this field.
/// *   - all indicates that the certificate is entitled to issue all values of
/// * the extension.
/// *   - specific is used to specify which values of the extension may be
/// * issued in the case where all does not apply.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CertIssueExtension {
    pub id: ExtId,
    pub permissions: CertIssueExtensionPermissions,
}

impl CertIssueExtension {
    pub fn new(id: ExtId, permissions: CertIssueExtensionPermissions) -> Self {
        Self { id, permissions }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum CertRequestExtensionPermissions {
    Content(Any),
    All(()),
}

///*
/// * @brief This field contains an individual CertRequestExtension.
/// * CertRequestExtensions specified in this standard are drawn from the
/// * ASN.1 Information Object Set SetCertExtensions. This set, and its use in
/// * the CertRequestExtension type, is structured so that each
/// * CertRequestExtension is associated with a AppExtension and a
/// * CertRequestExtension and all are identified by the same id value. In this
/// * structure:
/// *
/// * @param id: identifies the extension type.
/// *
/// * @param permissions: indicates the permissions. Within this field.
/// *   - all indicates that the certificate is entitled to issue all values of
/// * the extension.
/// *   - specific is used to specify which values of the extension may be
/// * issued in the case where all does not apply.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CertRequestExtension {
    pub id: ExtId,
    pub permissions: CertRequestExtensionPermissions,
}

impl CertRequestExtension {
    pub fn new(id: ExtId, permissions: CertRequestExtensionPermissions) -> Self {
        Self { id, permissions }
    }
}

///***************************************************************************
///                Certificates and other Security Management                 
///***************************************************************************
///*
/// * @brief This structure is a profile of the structure CertificateBase which
/// * specifies the valid combinations of fields to transmit implicit and
/// * explicit certificates.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the CertificateBase.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Certificate(pub CertificateBase);

///*
/// * @brief The fields in this structure have the following meaning:
/// *
/// * @param version: contains the version of the certificate format. In this
/// * version of the data structures, this field is set to 3.
/// *
/// * @param type: states whether the certificate is implicit or explicit. This
/// * field is set to explicit for explicit certificates and to implicit for
/// * implicit certificates. See ExplicitCertificate and ImplicitCertificate for
/// * more details.
/// *
/// * @param issuer: identifies the issuer of the certificate.
/// *
/// * @param toBeSigned: is the certificate contents. This field is an input to
/// * the hash when generating or verifying signatures for an explicit
/// * certificate, or generating or verifying the public key from the
/// * reconstruction value for an implicit certificate. The details of how this
/// * field are encoded are given in the description of the
/// * ToBeSignedCertificate type.
/// *
/// * @param signature: is included in an ExplicitCertificate. It is the
/// * signature, calculated by the signer identified in the issuer field, over
/// * the hash of toBeSigned. The hash is calculated as specified in 5.3.1, where:
/// *   - Data input is the encoding of toBeSigned following the COER.
/// *   - Signer identifier input depends on the verification type, which in
/// * turn depends on the choice indicated by issuer. If the choice indicated by
/// * issuer is self, the verification type is self-signed and the signer
/// * identifier input is the empty string. If the choice indicated by issuer is
/// * not self, the verification type is certificate and the signer identifier
/// * input is the canonicalized COER encoding of the certificate indicated by
/// * issuer. The canonicalization is carried out as specified in the
/// * Canonicalization section of this subclause.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the ToBeSignedCertificate and to the Signature.
/// *
/// * @note Whole-certificate hash: If the entirety of a certificate is hashed
/// * to calculate a HashedId3, HashedId8, or HashedId10, the algorithm used for
/// * this purpose is known as the whole-certificate hash. The method used to
/// * determine the whole-certificate hash algorithm is specified in 5.3.9.2.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CertificateBase {
    #[rasn(value("3"))]
    pub version: Uint8,
    pub r_type: CertificateType,
    pub issuer: IssuerIdentifier,
    pub to_be_signed: ToBeSignedCertificate,
    pub signature: Option<Signature>,
}

impl CertificateBase {
    pub fn new(
        version: Uint8,
        r_type: CertificateType,
        issuer: IssuerIdentifier,
        to_be_signed: ToBeSignedCertificate,
        signature: Option<Signature>,
    ) -> Self {
        Self {
            version,
            r_type,
            issuer,
            to_be_signed,
            signature,
        }
    }
}

///*
/// * @brief This structure contains information that is used to identify the
/// * certificate holder if necessary.
/// *
/// * @param linkageData: is used to identify the certificate for revocation
/// * purposes in the case of certificates that appear on linked certificate
/// * CRLs. See 5.1.3 and 7.3 for further discussion.
/// *
/// * @param name: is used to identify the certificate holder in the case of
/// * non-anonymous certificates. The contents of this field are a matter of
/// * policy and are expected to be human-readable.
/// *
/// * @param binaryId: supports identifiers that are not human-readable.
/// *
/// * @param none: indicates that the certificate does not include an identifier.
/// *
/// * @note Critical information fields:
/// *   - If present, this is a critical information field as defined in 5.2.6.
/// * An implementation that does not recognize the choice indicated in this
/// * field shall reject a signed SPDU as invalid.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum CertificateId {
    LinkageData(LinkageData),
    Name(Hostname),
    #[rasn(size("1..=64"))]
    BinaryId(OctetString),
    None(()),
}

///*
/// * @brief This enumerated type indicates whether a certificate is explicit or
/// * implicit.
/// *
/// * @note Critical information fields: If present, this is a critical
/// * information field as defined in 5.2.5. An implementation that does not
/// * recognize the indicated CHOICE for this type when verifying a signed SPDU
/// * shall indicate that the signed SPDU is invalid in the sense of 4.2.2.3.2,
/// * that is, it is invalid in the sense that its validity cannot be
/// * established.
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum CertificateType {
    Explicit = 0,
    Implicit = 1,
}

/// Anonymous SEQUENCE OF member

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct AnonymousContributedExtensionBlockExtns(Any);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1.."))]
pub struct ContributedExtensionBlockExtns(pub SequenceOf<AnonymousContributedExtensionBlockExtns>);

///*
/// * @brief This data structure defines the format of an extension block
/// * provided by an identified contributor by using the temnplate provided
/// * in the class IEEE1609DOT2-HEADERINFO-CONTRIBUTED-EXTENSION constraint
/// * to the objects in the set Ieee1609Dot2HeaderInfoContributedExtensions.
/// *
/// * @param contributorId: uniquely identifies the contributor.
/// *
/// * @param extns: contains a list of extensions from that contributor.
/// * Extensions are expected and not required to follow the format specified
/// * in 6.5.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ContributedExtensionBlock {
    pub contributor_id: HeaderInfoContributorId,
    pub extns: ContributedExtensionBlockExtns,
}

impl ContributedExtensionBlock {
    pub fn new(
        contributor_id: HeaderInfoContributorId,
        extns: ContributedExtensionBlockExtns,
    ) -> Self {
        Self {
            contributor_id,
            extns,
        }
    }
}

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1.."))]
pub struct ContributedExtensionBlocks(pub SequenceOf<ContributedExtensionBlock>);

///*
/// * @brief This data structure is used to perform a countersignature over an
/// * already-signed SPDU. This is the profile of an Ieee1609Dot2Data containing
/// * a signedData. The tbsData within content is composed of a payload
/// * containing the hash (extDataHash) of the externally generated, pre-signed
/// * SPDU over which the countersignature is performed.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Countersignature(pub Ieee1609Dot2Data);

///***************************************************************************
///                              Encrypted Data                               
///***************************************************************************
///*
/// * @brief This data structure encodes data that has been encrypted to one or
/// * more recipients using the recipients public or symmetric keys as
/// * specified in 5.3.4.
/// *
/// * @param recipients: contains one or more RecipientInfos. These entries may
/// * be more than one RecipientInfo, and more than one type of RecipientInfo,
/// * as long as all entries are indicating or containing the same data encryption
/// * key.
/// *
/// * @param ciphertext: contains the encrypted data. This is the encryption of
/// * an encoded Ieee1609Dot2Data structure as specified in 5.3.4.2.
/// *
/// * @note Critical information fields:
/// *   - If present, recipients is a critical information field as defined in
/// * 5.2.6. An implementation that does not support the number of RecipientInfo
/// * in recipients when decrypted shall indicate that the encrypted SPDU could
/// * not be decrypted due to unsupported critical information fields. A
/// * compliant implementation shall support recipients fields containing at
/// * least eight entries.
/// *
/// * @note If the plaintext is raw data, i.e., it has not been output from a
/// * previous operation of the SDS, then it is trivial to encapsulate it in an
/// * Ieee1609Dot2Data of type unsecuredData as noted in 4.2.2.2.2. For example,
/// * '03 80 08 01 23 45 67 89 AB CD EF' is the C-OER encoding of '01 23 45 67
/// * 89 AB CD EF' encapsulated in an Ieee1609Dot2Data of type unsecuredData.
/// * The first byte of the encoding 03 is the protocolVersion, the second byte
/// * 80 indicates the choice unsecuredData, and the third byte 08 is the length
/// * of the raw data '01 23 45 67 89 AB CD EF'.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EncryptedData {
    pub recipients: SequenceOfRecipientInfo,
    pub ciphertext: SymmetricCiphertext,
}

impl EncryptedData {
    pub fn new(recipients: SequenceOfRecipientInfo, ciphertext: SymmetricCiphertext) -> Self {
        Self {
            recipients,
            ciphertext,
        }
    }
}

///*
/// * @brief This data structure contains an encrypted data encryption key,
/// * where the data encryption key is input to the data encryption key
/// * encryption process with no headers, encapsulation, or length indication.
/// *
/// * Critical information fields: If present and applicable to
/// * the receiving SDEE, this is a critical information field as defined in
/// * 5.2.6. If an implementation receives an encrypted SPDU and determines that
/// * one or more RecipientInfo fields are relevant to it, and if all of those
/// * RecipientInfos contain an EncryptedDataEncryptionKey such that the
/// * implementation does not recognize the indicated CHOICE, the implementation
/// * shall indicate that the encrypted SPDU is not decryptable.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum EncryptedDataEncryptionKey {
    EciesNistP256(EciesP256EncryptedKey),
    EciesBrainpoolP256r1(EciesP256EncryptedKey),
    #[rasn(extension_addition)]
    EcencSm2256(EcencP256EncryptedKey),
}

///*
/// * @brief This type indicates which type of permissions may appear in
/// * end-entity certificates the chain of whose permissions passes through the
/// * PsidGroupPermissions field containing this value. If app is indicated, the
/// * end-entity certificate may contain an appPermissions field. If enroll is
/// * indicated, the end-entity certificate may contain a certRequestPermissions
/// * field.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct EndEntityType(pub BitString);

///*
/// * @brief This is a profile of the CertificateBase structure providing all
/// * the fields necessary for an explicit certificate, and no others.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ExplicitCertificate(pub CertificateBase);

///*
/// * @brief This structure contains the hash of some data with a specified hash
/// * algorithm. See 5.3.3 for specification of the permitted hash algorithms.
/// *
/// * @param sha256HashedData: indicates data hashed with SHA-256.
/// *
/// * @param sha384HashedData: indicates data hashed with SHA-384.
/// *
/// * @param sm3HashedData: indicates data hashed with SM3.
/// *
/// * @note Critical information fields: If present, this is a critical
/// * information field as defined in 5.2.6. An implementation that does not
/// * recognize the indicated CHOICE for this type when verifying a signed SPDU
/// * shall indicate that the signed SPDU is invalid in the sense of 4.2.2.3.2,
/// * that is, it is invalid in the sense that its validity cannot be established.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum HashedData {
    Sha256HashedData(HashedId32),
    #[rasn(extension_addition)]
    Sha384HashedData(HashedId48),
    #[rasn(extension_addition)]
    Sm3HashedData(HashedId32),
}

///*
/// * @brief This structure contains information that is used to establish
/// * validity by the criteria of 5.2.
/// *
/// * @param psid: indicates the application area with which the sender is
/// * claiming the payload is to be associated.
/// *
/// * @param generationTime: indicates the time at which the structure was
/// * generated. See 5.2.5.2.2 and 5.2.5.2.3 for discussion of the use of this
/// * field.
/// *
/// * @param expiryTime: if present, contains the time after which the data
/// * is no longer considered relevant. If both generationTime and
/// * expiryTime are present, the signed SPDU is invalid if generationTime is
/// * not strictly earlier than expiryTime.
/// *
/// * @param generationLocation: if present, contains the location at which the
/// * signature was generated.
/// *
/// * @param p2pcdLearningRequest: if present, is used by the SDS to request
/// * certificates for which it has seen identifiers and does not know the
/// * entire certificate. A specification of this peer-to-peer certificate
/// * distribution (P2PCD) mechanism is given in Clause 8. This field is used
/// * for the separate-certificate-pdu flavor of P2PCD and shall only be present
/// * if inlineP2pcdRequest is not present. The HashedId3 is calculated with the
/// * whole-certificate hash algorithm, determined as described in 6.4.3,
/// * applied to the COER-encoded certificate, canonicalized as defined in the
/// * definition of Certificate.
/// *
/// * @param missingCrlIdentifier: if present, is used by the SDS to request
/// * CRLs which it knows to have been issued and have not received. This is
/// * provided for future use and the associated mechanism is not defined in
/// * this version of this standard.
/// *
/// * @param encryptionKey: if present, is used to provide a key that is to
/// * be used to encrypt at least one response to this SPDU. The SDEE
/// * specification is expected to specify which response SPDUs are to be
/// * encrypted with this key. One possible use of this key to encrypt a
/// * response is specified in 6.3.35, 6.3.37, and 6.3.34. An encryptionKey
/// * field of type symmetric should only be used if the SignedData containing
/// * this field is securely encrypted by some means.
/// *
/// * @param inlineP2pcdRequest: if present, is used by the SDS to request
/// * unknown certificates per the inline peer-to-peer certificate distribution
/// * mechanism is given in Clause 8. This field shall only be present if
/// * p2pcdLearningRequest is not present. The HashedId3 is calculated with the
/// * whole-certificate hash algorithm, determined as described in 6.4.3, applied
/// * to the COER-encoded certificate, canonicalized as defined in the definition
/// * of Certificate.
/// *
/// * @param requestedCertificate: if present, is used by the SDS to provide
/// * certificates per the "inline" version of the peer-to-peer certificate
/// * distribution mechanism given in Clause 8.
/// *
/// * @param pduFunctionalType: if present, is used to indicate that the SPDU is
/// * to be consumed by a process other than an application process as defined
/// * in ISO 21177 [B14a]. See 6.3.23b for more details.
/// *
/// * @param contributedExtensions: if present, is used to contain additional
/// * extensions defined using the ContributedExtensionBlocks structure.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the EncryptionKey. If encryptionKey is present, and indicates
/// * the choice public, and contains a BasePublicEncryptionKey that is an
/// * elliptic curve point (i.e., of type EccP256CurvePoint or
/// * EccP384CurvePoint), then the elliptic curve point is encoded in compressed
/// * form, i.e., such that the choice indicated within the Ecc*CurvePoint is
/// * compressed-y-0 or compressed-y-1.
/// * The canonicalization does not apply to any fields after the extension
/// * marker, including any fields in contributedExtensions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct HeaderInfo {
    pub psid: Psid,
    pub generation_time: Option<Time64>,
    pub expiry_time: Option<Time64>,
    pub generation_location: Option<ThreeDLocation>,
    pub p2pcd_learning_request: Option<HashedId3>,
    pub missing_crl_identifier: Option<MissingCrlIdentifier>,
    pub encryption_key: Option<EncryptionKey>,
    #[rasn(extension_addition)]
    pub inline_p2pcd_request: Option<SequenceOfHashedId3>,
    #[rasn(extension_addition)]
    pub requested_certificate: Option<Certificate>,
    #[rasn(extension_addition)]
    pub pdu_functional_type: Option<PduFunctionalType>,
    #[rasn(extension_addition)]
    pub contributed_extensions: Option<ContributedExtensionBlocks>,
}

impl HeaderInfo {
    pub fn new(
        psid: Psid,
        generation_time: Option<Time64>,
        expiry_time: Option<Time64>,
        generation_location: Option<ThreeDLocation>,
        p2pcd_learning_request: Option<HashedId3>,
        missing_crl_identifier: Option<MissingCrlIdentifier>,
        encryption_key: Option<EncryptionKey>,
        inline_p2pcd_request: Option<SequenceOfHashedId3>,
        requested_certificate: Option<Certificate>,
        pdu_functional_type: Option<PduFunctionalType>,
        contributed_extensions: Option<ContributedExtensionBlocks>,
    ) -> Self {
        Self {
            psid,
            generation_time,
            expiry_time,
            generation_location,
            p2pcd_learning_request,
            missing_crl_identifier,
            encryption_key,
            inline_p2pcd_request,
            requested_certificate,
            pdu_functional_type,
            contributed_extensions,
        }
    }
}

///*
/// * @brief This is an integer used to identify a HeaderInfo extension
/// * contributing organization. In this version of this standard two values are
/// * defined:
/// *   - ieee1609OriginatingExtensionId indicating extensions originating with
/// * IEEE 1609.
/// *   - etsiOriginatingExtensionId indicating extensions originating with
/// * ETSI TC ITS.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HeaderInfoContributorId(pub u8);

///*
/// * @brief This structure uses the parameterized type Extension to define an
/// * Ieee1609ContributedHeaderInfoExtension as an open Extension Content field
/// * identified by an extension identifier. The extension identifier value is
/// * unique to extensions defined by ETSI and need not be unique among all
/// * extension identifier values defined by all contributing organizations.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Ieee1609ContributedHeaderInfoExtension(pub Extension);

///*
/// * @brief In this structure:
/// *
/// * @param unsecuredData: indicates that the content is an OCTET STRING to be
/// * consumed outside the SDS.
/// *
/// * @param signedData: indicates that the content has been signed according to
/// * this standard.
/// *
/// * @param encryptedData: indicates that the content has been encrypted
/// * according to this standard.
/// *
/// * @param signedCertificateRequest: indicates that the content is a
/// * certificate request signed by an IEEE 1609.2 certificate or self-signed.
/// *
/// * @param signedX509CertificateRequest: indicates that the content is a
/// * certificate request signed by an ITU-T X.509 certificate.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2 if it is of type signedData.
/// * The canonicalization applies to the SignedData.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum Ieee1609Dot2Content {
    UnsecuredData(Opaque),
    SignedData(alloc::boxed::Box<SignedData>),
    EncryptedData(EncryptedData),
    SignedCertificateRequest(Opaque),
    #[rasn(extension_addition)]
    SignedX509CertificateRequest(Opaque),
}

///***************************************************************************
///                               Secured Data                                
///***************************************************************************
///*
/// * @brief This data type is used to contain the other data types in this
/// * clause. The fields in the Ieee1609Dot2Data have the following meanings:
/// *
/// * @param protocolVersion: contains the current version of the protocol. The
/// * version specified in this standard is version 3, represented by the
/// * integer 3. There are no major or minor version numbers.
/// *
/// * @param content: contains the content in the form of an Ieee1609Dot2Content.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the Ieee1609Dot2Content.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Ieee1609Dot2Data {
    #[rasn(value("3"))]
    pub protocol_version: Uint8,
    pub content: Ieee1609Dot2Content,
}

impl Ieee1609Dot2Data {
    pub fn new(protocol_version: Uint8, content: Ieee1609Dot2Content) -> Self {
        Self {
            protocol_version,
            content,
        }
    }
}

///*
/// * @brief This is an integer used to identify an
/// * Ieee1609ContributedHeaderInfoExtension.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Ieee1609HeaderInfoExtensionId(pub ExtId);

///*
/// * @brief This is a profile of the CertificateBase structure providing all
/// * the fields necessary for an implicit certificate, and no others.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ImplicitCertificate(pub CertificateBase);

///*
/// * @brief This structure allows the recipient of a certificate to determine
/// * which keying material to use to authenticate the certificate.
/// *
/// * If the choice indicated is sha256AndDigest, sha384AndDigest, or
/// * sm3AndDigest:
/// *   - The structure contains the HashedId8 of the issuing certificate. The
/// * HashedId8 is calculated with the whole-certificate hash algorithm,
/// * determined as described in 6.4.3, applied to the COER-encoded certificate,
/// * canonicalized as defined in the definition of Certificate.
/// *   - The hash algorithm to be used to generate the hash of the certificate
/// * for verification is SHA-256 (in the case of sha256AndDigest), SM3 (in the
/// * case of sm3AndDigest) or SHA-384 (in the case of sha384AndDigest).
/// *   - The certificate is to be verified with the public key of the
/// * indicated issuing certificate.
/// *
/// * If the choice indicated is self:
/// *   - The structure indicates what hash algorithm is to be used to generate
/// * the hash of the certificate for verification.
/// *   - The certificate is to be verified with the public key indicated by
/// * the verifyKeyIndicator field in theToBeSignedCertificate.
/// *
/// * @note Critical information fields: If present, this is a critical
/// * information field as defined in 5.2.5. An implementation that does not
/// * recognize the indicated CHOICE for this type when verifying a signed SPDU
/// * shall indicate that the signed SPDU is invalid in the sense of 4.2.2.3.2,
/// * that is, it is invalid in the sense that its validity cannot be
/// * established.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum IssuerIdentifier {
    Sha256AndDigest(HashedId8),
    RsSelf(HashAlgorithm),
    #[rasn(extension_addition)]
    Sha384AndDigest(HashedId8),
    #[rasn(extension_addition)]
    Sm3AndDigest(HashedId8),
}

///*
/// * @brief This structure contains information that is matched against
/// * information obtained from a linkage ID-based CRL to determine whether the
/// * containing certificate has been revoked. See 5.1.3.4 and 7.3 for details
/// * of use.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LinkageData {
    pub i_cert: IValue,
    pub linkage_value: LinkageValue,
    pub group_linkage_value: Option<GroupLinkageValue>,
}

impl LinkageData {
    pub fn new(
        i_cert: IValue,
        linkage_value: LinkageValue,
        group_linkage_value: Option<GroupLinkageValue>,
    ) -> Self {
        Self {
            i_cert,
            linkage_value,
            group_linkage_value,
        }
    }
}

///*
/// * @brief This structure may be used to request a CRL that the SSME knows to
/// * have been issued and has not yet received. It is provided for future use
/// * and its use is not defined in this version of this standard.
/// *
/// * @param cracaId: is the HashedId3 of the CRACA, as defined in 5.1.3. The
/// * HashedId3 is calculated with the whole-certificate hash algorithm,
/// * determined as described in 6.4.3, applied to the COER-encoded certificate,
/// * canonicalized as defined in the definition of Certificate.
/// *
/// * @param crlSeries: is the requested CRL Series value. See 5.1.3 for more
/// * information.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MissingCrlIdentifier {
    pub craca_id: HashedId3,
    pub crl_series: CrlSeries,
}

impl MissingCrlIdentifier {
    pub fn new(craca_id: HashedId3, crl_series: CrlSeries) -> Self {
        Self {
            craca_id,
            crl_series,
        }
    }
}

///*
/// * @brief This data structure encapsulates an encrypted ciphertext for any
/// * symmetric algorithm with 128-bit blocks in CCM mode. The ciphertext is
/// * 16 bytes longer than the corresponding plaintext due to the inclusion of
/// * the message authentication code (MAC). The plaintext resulting from a
/// * correct decryption of the ciphertext is either a COER-encoded
/// * Ieee1609Dot2Data structure (see 6.3.41), or a 16-byte symmetric key
/// * (see 6.3.44).
/// *
/// * The ciphertext is 16 bytes longer than the corresponding plaintext.
/// *
/// * The plaintext resulting from a correct decryption of the
/// * ciphertext is a COER-encoded Ieee1609Dot2Data structure.
/// *
/// * @param nonce: contains the nonce N as specified in 5.3.8.
/// *
/// * @param ccmCiphertext: contains the ciphertext C as specified in 5.3.8.
/// *
/// * @note In the name of this structure, "One28" indicates that the
/// * symmetric cipher block size is 128 bits. It happens to also be the case
/// * that the keys used for both AES-128-CCM and SM4-CCM are also 128 bits long.
/// * This is, however, not what One28 refers to. Since the cipher is used in
/// * counter mode, i.e., as a stream cipher, the fact that that block size is 128
/// * bits affects only the size of the MAC and does not affect the size of the
/// * raw ciphertext.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct One28BitCcmCiphertext {
    #[rasn(size("12"))]
    pub nonce: OctetString,
    pub ccm_ciphertext: Opaque,
}

impl One28BitCcmCiphertext {
    pub fn new(nonce: OctetString, ccm_ciphertext: Opaque) -> Self {
        Self {
            nonce,
            ccm_ciphertext,
        }
    }
}

///*
/// * @brief This type is the AppExtension used to identify an operating
/// * organization. The associated CertIssueExtension and CertRequestExtension
/// * are both of type OperatingOrganizationId.
/// * To determine consistency between this type and an SPDU, the SDEE
/// * specification for that SPDU is required to specify how the SPDU can be
/// * used to determine an OBJECT IDENTIFIER (for example, by including the
/// * full OBJECT IDENTIFIER in the SPDU, or by including a RELATIVE-OID with
/// * clear instructions about how a full OBJECT IDENTIFIER can be obtained from
/// * the RELATIVE-OID). The SPDU is then consistent with this type if the
/// * OBJECT IDENTIFIER determined from the SPDU is identical to the OBJECT
/// * IDENTIFIER contained in this field.
/// * This AppExtension does not have consistency conditions with a
/// * corresponding CertIssueExtension. It can appear in a certificate issued
/// * by any CA.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct OperatingOrganizationId(pub ObjectIdentifier);

///*
/// * @brief This data structure contains the following fields:
/// *
/// * @param recipientId: contains the hash of the container for the encryption
/// * public key as specified in the definition of RecipientInfo. Specifically,
/// * depending on the choice indicated by the containing RecipientInfo structure:
/// *   - If the containing RecipientInfo structure indicates certRecipInfo,
/// * this field contains the HashedId8 of the certificate. The HashedId8 is
/// * calculated with the whole-certificate hash algorithm, determined as
/// * described in 6.4.3, applied to the COER-encoded certificate, canonicalized
/// * as defined in the definition of Certificate.
/// *   - If the containing RecipientInfo structure indicates
/// * signedDataRecipInfo, this field contains the HashedId8 of the
/// * Ieee1609Dot2Data of type signedData that contained the encryption key,
/// * with that Ieee¬¬1609¬Dot2¬¬Data canonicalized per 6.3.4. The HashedId8 is
/// * calculated with the hash algorithm determined as specified in 5.3.9.5.
/// *   - If the containing RecipientInfo structure indicates rekRecipInfo, this
/// * field contains the HashedId8 of the COER encoding of a PublicEncryptionKey
/// * structure containing the response encryption key. The HashedId8 is
/// * calculated with the hash algorithm determined as specified in 5.3.9.5.
/// *
/// * @param encKey: contains the encrypted data encryption key, where the data
/// * encryption key is input to the data encryption key encryption process with
/// * no headers, encapsulation, or length indication.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PKRecipientInfo {
    pub recipient_id: HashedId8,
    pub enc_key: EncryptedDataEncryptionKey,
}

impl PKRecipientInfo {
    pub fn new(recipient_id: HashedId8, enc_key: EncryptedDataEncryptionKey) -> Self {
        Self {
            recipient_id,
            enc_key,
        }
    }
}

///*
/// * @brief This data structure identifies the functional entity that is
/// * intended to consume an SPDU, for the case where that functional entity is
/// * not an application process, and are instead security support services for an
/// * application process. Further details and the intended use of this field are
/// * defined in ISO 21177 [B20].
/// *
/// * @param tlsHandshake: indicates that the Signed SPDU is not to be directly
/// * consumed as an application PDU and is to be used to provide information
/// * about the holders permissions to a Transport Layer Security (TLS)
/// * (IETF 5246 [B15], IETF 8446 [B16]) handshake process operating to secure
/// * communications to an application process. See IETF [B15] and ISO 21177
/// * [B20] for further information.
/// *
/// * @param iso21177ExtendedAuth: indicates that the Signed SPDU is not to be
/// * directly consumed as an application PDU and is to be used to provide
/// * additional information about the holders permissions to the ISO 21177
/// * Security Subsystem for an application process. See ISO 21177 [B20] for
/// * further information.
/// *
/// * @param iso21177SessionExtension: indicates that the Signed SPDU is not to
/// * be directly consumed as an application PDU and is to be used to extend an
/// * existing ISO 21177 secure session. This enables a secure session to
/// * persist beyond the lifetime of the certificates used to establish that
/// * session.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct PduFunctionalType(pub u8);

///*
/// * @brief This data structure is used to indicate a symmetric key that may
/// * be used directly to decrypt a SymmetricCiphertext. It consists of the
/// * low-order 8 bytes of the hash of the COER encoding of a
/// * SymmetricEncryptionKey structure containing the symmetric key in question.
/// * The HashedId8 is calculated with the hash algorithm determined as
/// * specified in 5.3.9.3. The symmetric key may be established by any
/// * appropriate means agreed by the two parties to the exchange.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct PreSharedKeyRecipientInfo(pub HashedId8);

///*
/// * @brief This structure states the permissions that a certificate holder has
/// * with respect to issuing and requesting certificates for a particular set
/// * of PSIDs. For examples, see D.5.3 and D.5.4.
/// *
/// * @param subjectPermissions: indicates PSIDs and SSP Ranges covered by this
/// * field.
/// *
/// * @param minChainLength: and chainLengthRange indicate how long the
/// * certificate chain from this certificate to the end-entity certificate is
/// * permitted to be. As specified in 5.1.2.1, the length of the certificate
/// * chain is the number of certificates "below" this certificate in the chain,
/// * down to and including the end-entity certificate. The length is permitted
/// * to be (a) greater than or equal to minChainLength certificates and (b)
/// * less than or equal to minChainLength + chainLengthRange certificates. A
/// * value of 0 for minChainLength is not permitted when this type appears in
/// * the certIssuePermissions field of a ToBeSignedCertificate; a certificate
/// * that has a value of 0 for this field is invalid. The value -1 for
/// * chainLengthRange is a special case: if the value of chainLengthRange is -1
/// * it indicates that the certificate chain may be any length equal to or
/// * greater than minChainLength. See the examples below for further discussion.
/// *
/// * @param eeType: takes one or more of the values app and enroll and indicates
/// * the type of certificates or requests that this instance of
/// * PsidGroupPermissions in the certificate is entitled to authorize.
/// * Different instances of PsidGroupPermissions within a ToBeSignedCertificate
/// * may have different values for eeType.
/// *   - If this field indicates app, the chain is allowed to end in an
/// * authorization certificate, i.e., a certficate in which these permissions
/// * appear in an appPermissions field (in other words, if the field does not
/// * indicate app and the chain ends in an authorization certificate, the
/// * chain shall be considered invalid).
/// *   - If this field indicates enroll, the chain is allowed to end in an
/// * enrollment certificate, i.e., a certificate in which these permissions
/// * appear in a certReqPermissions permissions field (in other words, if the
/// * field does not indicate enroll and the chain ends in an enrollment
/// * certificate, the chain shall be considered invalid).
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PsidGroupPermissions {
    pub subject_permissions: SubjectPermissions,
    #[rasn(default = "psid_group_permissions_min_chain_length_default")]
    pub min_chain_length: Integer,
    #[rasn(default = "psid_group_permissions_chain_length_range_default")]
    pub chain_length_range: Integer,
    #[rasn(default = "psid_group_permissions_ee_type_default")]
    pub ee_type: EndEntityType,
}

impl PsidGroupPermissions {
    pub fn new(
        subject_permissions: SubjectPermissions,
        min_chain_length: Integer,
        chain_length_range: Integer,
        ee_type: EndEntityType,
    ) -> Self {
        Self {
            subject_permissions,
            min_chain_length,
            chain_length_range,
            ee_type,
        }
    }
}

fn psid_group_permissions_min_chain_length_default() -> Integer {
    1.into()
}

fn psid_group_permissions_chain_length_range_default() -> Integer {
    0.into()
}

fn psid_group_permissions_ee_type_default() -> EndEntityType {
    EndEntityType(
        [true, false, false, false, false, false, false]
            .into_iter()
            .collect(),
    )
    .into()
}

///*
/// * @brief This data structure is used to transfer the data encryption key to
/// * an individual recipient of an EncryptedData. The option pskRecipInfo is
/// * selected if the EncryptedData was encrypted using the static encryption
/// * key approach specified in 5.3.4. The other options are selected if the
/// * EncryptedData was encrypted using the ephemeral encryption key approach
/// * specified in 5.3.4. The meanings of the choices are:
/// *
/// * See Annex C.7 for guidance on when it may be appropriate to use
/// * each of these approaches.
/// *
/// * @param pskRecipInfo: The data was encrypted directly using a pre-shared
/// * symmetric key.
/// *
/// * @param symmRecipInfo: The data was encrypted with a data encryption key,
/// * and the data encryption key was encrypted using a symmetric key.
/// *
/// * @param certRecipInfo: The data was encrypted with a data encryption key,
/// * the data encryption key was encrypted using a public key encryption scheme,
/// * where the public encryption key was obtained from a certificate. In this
/// * case, the parameter P1 to ECIES as defined in 5.3.5 is the hash of the
/// * certificate, calculated with the whole-certificate hash algorithm,
/// * determined as described in 6.4.3, applied to the COER-encoded certificate,
/// * canonicalized as defined in the definition of Certificate.
/// *
/// * @note If the encryption algorithm is SM2, there is no equivalent of the
/// * parameter P1 and so no input to the encryption process that uses the hash
/// * of the certificate.
/// *
/// * @param signedDataRecipInfo: The data was encrypted with a data encryption
/// * key, the data encryption key was encrypted using a public key encryption
/// * scheme, where the public encryption key was obtained as the public response
/// * encryption key from a SignedData. In this case, if ECIES is the encryption
/// * algorithm, then the parameter P1 to ECIES as defined in 5.3.5 is the
/// * SHA-256 hash of the Ieee1609Dot2Data of type signedData containing the
/// * response encryption key, canonicalized as defined in the definition of
/// * Ieee1609Dot2Data.
/// *
/// * @note If the encryption algorithm is SM2, there is no equivalent of the
/// * parameter P1 and so no input to the encryption process that uses the hash
/// * of the Ieee1609Dot2Data.
/// *
/// * @param rekRecipInfo: The data was encrypted with a data encryption key,
/// * the data encryption key was encrypted using a public key encryption scheme,
/// * where the public encryption key was not obtained from a Signed-Data or a
/// * certificate. In this case, the SDEE specification is expected to specify
/// * how the public key is obtained, and if ECIES is the encryption algorithm,
/// * then the parameter P1 to ECIES as defined in 5.3.5 is the hash of the
/// * empty string.
/// *
/// * @note If the encryption algorithm is SM2, there is no equivalent of the
/// * parameter P1 and so no input to the encryption process that uses the hash
/// * of the empty string.
/// *
/// * @note The material input to encryption is the bytes of the encryption key
/// * with no headers, encapsulation, or length indication. Contrast this to
/// * encryption of data, where the data is encapsulated in an Ieee1609Dot2Data.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum RecipientInfo {
    PskRecipInfo(PreSharedKeyRecipientInfo),
    SymmRecipInfo(SymmRecipientInfo),
    CertRecipInfo(PKRecipientInfo),
    SignedDataRecipInfo(PKRecipientInfo),
    RekRecipInfo(PKRecipientInfo),
}

///*
/// * @brief This structure contains any AppExtensions that apply to the
/// * certificate holder. As specified in 5.2.4.2.3, each individual
/// * AppExtension type is associated with consistency conditions, specific to
/// * that extension, that govern its consistency with SPDUs signed by the
/// * certificate holder and with the CertIssueExtensions in the CA certificates
/// * in that certificate holders chain. Those consistency conditions are
/// * specified for each individual AppExtension below.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1.."))]
pub struct SequenceOfAppExtensions(pub SequenceOf<AppExtension>);

///*
/// * @brief This field contains any CertIssueExtensions that apply to the
/// * certificate holder. As specified in 5.2.4.2.3, each individual
/// * CertIssueExtension type is associated with consistency conditions,
/// * specific to that extension, that govern its consistency with
/// * AppExtensions in certificates issued by the certificate holder and with
/// * the CertIssueExtensions in the CA certificates in that certificate
/// * holders chain. Those consistency conditions are specified for each
/// * individual CertIssueExtension below.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1.."))]
pub struct SequenceOfCertIssueExtensions(pub SequenceOf<CertIssueExtension>);

///*
/// * @brief This field contains any CertRequestExtensions that apply to the
/// * certificate holder. As specified in 5.2.4.2.3, each individual
/// * CertRequestExtension type is associated with consistency conditions,
/// * specific to that extension, that govern its consistency with
/// * AppExtensions in certificates issued by the certificate holder and with
/// * the CertRequestExtensions in the CA certificates in that certificate
/// * holders chain. Those consistency conditions are specified for each
/// * individual CertRequestExtension below.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1.."))]
pub struct SequenceOfCertRequestExtensions(pub SequenceOf<CertRequestExtension>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfCertificate(pub SequenceOf<Certificate>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfPsidGroupPermissions(pub SequenceOf<PsidGroupPermissions>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfRecipientInfo(pub SequenceOf<RecipientInfo>);

///*
/// * @brief In this structure:
/// *
/// * @param hashId: indicates the hash algorithm to be used to generate the hash
/// * of the message for signing and verification.
/// *
/// * @param tbsData: contains the data that is hashed as input to the signature.
/// *
/// * @param signer: determines the keying material and hash algorithm used to
/// * sign the data.
/// *
/// * @param signature: contains the digital signature itself, calculated as
/// * specified in 5.3.1.
/// *   - If signer indicates the choice self, then the signature calculation
/// * is parameterized as follows:
/// *     - Data input is equal to the COER encoding of the tbsData field
/// * canonicalized according to the encoding considerations given in 6.3.6.
/// *     - Verification type is equal to self.
/// *     - Signer identifier input is equal to the empty string.
/// *   - If signer indicates certificate or digest, then the signature
/// * calculation is parameterized as follows:
/// *     - Data input is equal to the COER encoding of the tbsData field
/// * canonicalized according to the encoding considerations given in 6.3.6.
/// *     - Verification type is equal to certificate.
/// *     - Signer identifier input equal to the COER-encoding of the
/// * Certificate that is to be used to verify the SPDU, canonicalized according
/// * to the encoding considerations given in 6.4.3.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the ToBeSignedData and the Signature.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SignedData {
    pub hash_id: HashAlgorithm,
    pub tbs_data: ToBeSignedData,
    pub signer: SignerIdentifier,
    pub signature: Signature,
}

impl SignedData {
    pub fn new(
        hash_id: HashAlgorithm,
        tbs_data: ToBeSignedData,
        signer: SignerIdentifier,
        signature: Signature,
    ) -> Self {
        Self {
            hash_id,
            tbs_data,
            signer,
            signature,
        }
    }
}

///*
/// * @brief This structure contains the data payload of a ToBeSignedData. This
/// * structure contains at least one of the optional elements, and may contain
/// * more than one. See 5.2.4.3.4 for more details.
/// * The security profile in Annex C allows an implementation of this standard
/// * to state which forms of Signed¬Data¬Payload are supported by that
/// * implementation, and also how the signer and verifier are intended to obtain
/// * the external data for hashing. The specification of an SDEE that uses
/// * external data is expected to be explicit and unambiguous about how this
/// * data is obtained and how it is formatted prior to processing by the hash
/// * function.
/// *
/// * @param data: contains data that is explicitly transported within the
/// * structure.
/// *
/// * @param extDataHash: contains the hash of data that is not explicitly
/// * transported within the structure, and which the creator of the structure
/// * wishes to cryptographically bind to the signature.
/// *
/// * @param omitted: indicates that there is external data to be included in the
/// * hash calculation for the signature.The mechanism for including the external
/// * data in the hash calculation is specified in 6.3.6.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the Ieee1609Dot2Data.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignedDataPayload {
    pub data: Option<Ieee1609Dot2Data>,
    pub ext_data_hash: Option<HashedData>,
    #[rasn(extension_addition)]
    pub omitted: Option<()>,
}

impl SignedDataPayload {
    pub fn new(
        data: Option<Ieee1609Dot2Data>,
        ext_data_hash: Option<HashedData>,
        omitted: Option<()>,
    ) -> Self {
        Self {
            data,
            ext_data_hash,
            omitted,
        }
    }
}

///*
/// * @brief This structure allows the recipient of data to determine which
/// * keying material to use to authenticate the data. It also indicates the
/// * verification type to be used to generate the hash for verification, as
/// * specified in 5.3.1.
/// *
/// * @param digest: If the choice indicated is digest:
/// *   - The structure contains the HashedId8 of the relevant certificate. The
/// * HashedId8 is calculated with the whole-certificate hash algorithm,
/// * determined as described in 6.4.3.
/// *   - The verification type is certificate and the certificate data
/// * passed to the hash function as specified in 5.3.1 is the authorization
/// * certificate.
/// *
/// * @param certificate: If the choice indicated is certificate:
/// *   - The structure contains one or more Certificate structures, in order
/// * such that the first certificate is the authorization certificate and each
/// * subsequent certificate is the issuer of the one before it.
/// *   - The verification type is certificate and the certificate data
/// * passed to the hash function as specified in 5.3.1 is the authorization
/// * certificate.
/// *
/// * @param self: If the choice indicated is self:
/// *   - The structure does not contain any data beyond the indication that
/// * the choice value is self.
/// *   - The verification type is self-signed.
/// *
/// * @note Critical information fields:
/// *   - If present, this is a critical information field as defined in 5.2.6.
/// * An implementation that does not recognize the CHOICE value for this type
/// * when verifying a signed SPDU shall indicate that the signed SPDU is invalid.
/// *   - If present, certificate is a critical information field as defined in
/// * 5.2.6. An implementation that does not support the number of certificates
/// * in certificate when verifying a signed SPDU shall indicate that the signed
/// * SPDU is invalid. A compliant implementation shall support certificate
/// * fields containing at least one certificate.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to every Certificate in the certificate field.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SignerIdentifier {
    Digest(HashedId8),
    Certificate(SequenceOfCertificate),
    RsSelf(()),
}

///*
/// * @brief This indicates the PSIDs and associated SSPs for which certificate
/// * issuance or request permissions are granted by a PsidGroupPermissions
/// * structure. If this takes the value explicit, the enclosing
/// * PsidGroupPermissions structure grants certificate issuance or request
/// * permissions for the indicated PSIDs and SSP Ranges. If this takes the
/// * value all, the enclosing PsidGroupPermissions structure grants certificate
/// * issuance or request permissions for all PSIDs not indicated by other
/// * PsidGroupPermissions in the same certIssuePermissions or
/// * certRequestPermissions field.
/// *
/// * @note Critical information fields:
/// *   - If present, this is a critical information field as defined in 5.2.6.
/// * An implementation that does not recognize the indicated CHOICE when
/// * verifying a signed SPDU shall indicate that the signed SPDU is
/// * invalidin the sense of 4.2.2.3.2, that is, it is invalid in the sense that
/// * its validity cannot be established.
/// *   - If present, explicit is a critical information field as defined in
/// * 5.2.6. An implementation that does not support the number of PsidSspRange
/// * in explicit when verifying a signed SPDU shall indicate that the signed
/// * SPDU is invalid in the sense of 4.2.2.3.2, that is, it is invalid in the
/// * sense that its validity cannot be established. A conformant implementation
/// * shall support explicit fields containing at least eight entries.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SubjectPermissions {
    Explicit(SequenceOfPsidSspRange),
    All(()),
}

///*
/// * @brief This data structure contains the following fields:
/// *
/// * @param recipientId: contains the hash of the symmetric key encryption key
/// * that may be used to decrypt the data encryption key. It consists of the
/// * low-order 8 bytes of the hash of the COER encoding of a
/// * SymmetricEncryptionKey structure containing the symmetric key in question.
/// * The HashedId8 is calculated with the hash algorithm determined as
/// * specified in 5.3.9.4. The symmetric key may be established by any
/// * appropriate means agreed by the two parties to the exchange.
/// *
/// * @param encKey: contains the encrypted data encryption key within a
/// * SymmetricCiphertext, where the data encryption key is input to the data
/// * encryption key encryption process with no headers, encapsulation, or
/// * length indication.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SymmRecipientInfo {
    pub recipient_id: HashedId8,
    pub enc_key: SymmetricCiphertext,
}

impl SymmRecipientInfo {
    pub fn new(recipient_id: HashedId8, enc_key: SymmetricCiphertext) -> Self {
        Self {
            recipient_id,
            enc_key,
        }
    }
}

///*
/// * @brief This data structure encapsulates a ciphertext generated with an
/// * approved symmetric algorithm.
/// *
/// * @note Critical information fields: If present, this is a critical
/// * information field as defined in 5.2.6. An implementation that does not
/// * recognize the indicated CHOICE value for this type in an encrypted SPDU
/// * shall indicate that the signed SPDU is invalid in the sense of 4.2.2.3.2,
/// * that is, it is invalid in the sense that its validity cannot be established.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SymmetricCiphertext {
    Aes128ccm(One28BitCcmCiphertext),
    #[rasn(extension_addition)]
    Sm4Ccm(One28BitCcmCiphertext),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct TestCertificate(pub Certificate);

///*
/// * @brief The fields in the ToBeSignedCertificate structure have the
/// * following meaning:
/// *
/// * For both implicit and explicit certificates, when the certificate
/// * is hashed to create or recover the public key (in the case of an implicit
/// * certificate) or to generate or verify the signature (in the case of an
/// * explicit certificate), the hash is Hash (Data input) || Hash (
/// * Signer identifier input), where:
/// *   - Data input is the COER encoding of toBeSigned, canonicalized
/// * as described above.
/// *   - Signer identifier input depends on the verification type,
/// * which in turn depends on the choice indicated by issuer. If the choice
/// * indicated by issuer is self, the verification type is self-signed and the
/// * signer identifier input is the empty string. If the choice indicated by
/// * issuer is not self, the verification type is certificate and the signer
/// * identifier input is the COER encoding of the canonicalization per 6.4.3 of
/// * the certificate indicated by issuer.
/// *
/// * In other words, for implicit certificates, the value H (CertU) in SEC 4,
/// * section 3, is for purposes of this standard taken to be H [H
/// * (canonicalized ToBeSignedCertificate from the subordinate certificate) ||
/// * H (entirety of issuer Certificate)]. See 5.3.2 for further discussion,
/// * including material differences between this standard and SEC 4 regarding
/// * how the hash function output is converted from a bit string to an integer.
/// *
/// * @param id: contains information that is used to identify the certificate
/// * holder if necessary.
/// *
/// * @param cracaId: identifies the Certificate Revocation Authorization CA
/// * (CRACA) responsible for certificate revocation lists (CRLs) on which this
/// * certificate might appear. Use of the cracaId is specified in 5.1.3. The
/// * HashedId3 is calculated with the whole-certificate hash algorithm,
/// * determined as described in 6.4.3, applied to the COER-encoded certificate,
/// * canonicalized as defined in the definition of Certificate.
/// *
/// * @param crlSeries: represents the CRL series relevant to a particular
/// * Certificate Revocation Authorization CA (CRACA) on which the certificate
/// * might appear. Use of this field is specified in 5.1.3.
/// *
/// * @param validityPeriod: contains the validity period of the certificate.
/// *
/// * @param region: if present, indicates the validity region of the
/// * certificate. If it is omitted the validity region is indicated as follows:
/// *   - If enclosing certificate is self-signed, i.e., the choice indicated
/// * by the issuer field in the enclosing certificate structure is self, the
/// * certificate is valid worldwide.
/// *   - Otherwise, the certificate has the same validity region as the
/// * certificate that issued it.
/// *
/// * @param assuranceLevel: indicates the assurance level of the certificate
/// * holder.
/// *
/// * @param appPermissions: indicates the permissions that the certificate
/// * holder has to sign application data with this certificate. A valid
/// * instance of appPermissions contains any particular Psid value in at most
/// * one entry.
/// *
/// * @param certIssuePermissions: indicates the permissions that the certificate
/// * holder has to sign certificates with this certificate. A valid instance of
/// * this array contains no more than one entry whose psidSspRange field
/// * indicates all. If the array has multiple entries and one entry has its
/// * psidSspRange field indicate all, then the entry indicating all specifies
/// * the permissions for all PSIDs other than the ones explicitly specified in
/// * the other entries. See the description of PsidGroupPermissions for further
/// * discussion.
/// *
/// * @param certRequestPermissions: indicates the permissions that the
/// * certificate holder can request in its certificate. A valid instance of this
/// * array contains no more than one entry whose psidSspRange field indicates
/// * all. If the array has multiple entries and one entry has its psidSspRange
/// * field indicate all, then the entry indicating all specifies the permissions
/// * for all PSIDs other than the ones explicitly specified in the other entries.
/// * See the description of PsidGroupPermissions for further discussion.
/// *
/// * @param canRequestRollover: indicates that the certificate may be used to
/// * sign a request for another certificate with the same permissions. This
/// * field is provided for future use and its use is not defined in this
/// * version of this standard.
/// *
/// * @param encryptionKey: contains a public key for encryption for which the
/// * certificate holder holds the corresponding private key.
/// *
/// * @param verifyKeyIndicator: contains material that may be used to recover
/// * the public key that may be used to verify data signed by this certificate.
/// *
/// * @param flags: indicates additional yes/no properties of the certificate
/// * holder. The only bit with defined semantics in this string in this version
/// * of this standard is usesCubk. If set, the usesCubk bit indicates that the
/// * certificate holder supports the compact unified butterfly key response.
/// * Further material about the compact unified butterfly key response can be
/// * found in IEEE Std 1609.2.1.
/// *
/// * @note usesCubk is only relevant for CA certificates, and the only
/// * functionality defined associated with this field is associated with
/// * consistency checks on received certificate responses. No functionality
/// * associated with communications between peer SDEEs is defined associated
/// * with this field.
/// *
/// * @param appExtensions: indicates additional permissions that may be applied
/// * to application activities that the certificate holder is carrying out.
/// *
/// * @param certIssueExtensions: indicates additional permissions to issue
/// * certificates containing endEntityExtensions.
/// *
/// * @param certRequestExtensions: indicates additional permissions to request
/// * certificates containing endEntityExtensions.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the PublicEncryptionKey and to the VerificationKeyIndicator.
/// *
/// * If the PublicEncryptionKey contains a BasePublicEncryptionKey that is an
/// * elliptic curve point (i.e., of type EccP256CurvePoint or EccP384CurvePoint),
/// * then the elliptic curve point is encoded in compressed form, i.e., such
/// * that the choice indicated within the Ecc*CurvePoint is compressed-y-0 or
/// * compressed-y-1.
/// *
/// * @note Critical information fields:
/// *   - If present, appPermissions is a critical information field as defined
/// * in 5.2.6. If an implementation of verification does not support the number
/// * of PsidSsp in the appPermissions field of a certificate that signed a
/// * signed SPDU, that implementation shall indicate that the signed SPDU is
/// * invalid in the sense of 4.2.2.3.2, that is, it is invalid in the sense
/// * that its validity cannot be established.. A conformant implementation
/// * shall support appPermissions fields containing at least eight entries.
/// * It may be the case that an implementation of verification does not support
/// * the number of entries in  the appPermissions field and the appPermissions
/// * field is not relevant to the verification: this will occur, for example,
/// * if the certificate in question is a CA certificate and so the
/// * certIssuePermissions field is relevant to the verification and the
/// * appPermissions field is not. In this case, whether the implementation
/// * indicates that the signed SPDU is valid (because it could validate all
/// * relevant fields) or invalid (because it could not parse the entire
/// * certificate) is implementation-specific.
/// *   - If present, certIssuePermissions is a critical information field as
/// * defined in 5.2.6. If an implementation of verification does not support
/// * the number of PsidGroupPermissions in the certIssuePermissions field of a
/// * CA certificate in the chain of a signed SPDU, the implementation shall
/// * indicate that the signed SPDU is invalid in the sense of 4.2.2.3.2, that
/// * is, it is invalid in the sense that its validity cannot be established.
/// * A conformant implementation shall support certIssuePermissions fields
/// * containing at least eight entries.
/// * It may be the case that an implementation of verification does not support
/// * the number of entries in  the certIssuePermissions field and the
/// * certIssuePermissions field is not relevant to the verification: this will
/// * occur, for example, if the certificate in question is the signing
/// * certificate for the SPDU and so the appPermissions field is relevant to
/// * the verification and the certIssuePermissions field is not. In this case,
/// * whether the implementation indicates that the signed SPDU is valid
/// * (because it could validate all relevant fields) or invalid (because it
/// * could not parse the entire certificate) is implementation-specific.
/// *   - If present, certRequestPermissions is a critical information field as
/// * defined in 5.2.6. If an implementaiton of verification of a certificate
/// * request does not support the number of PsidGroupPermissions in
/// * certRequestPermissions, the implementation shall indicate that the signed
/// * SPDU is invalid in the sense of 4.2.2.3.2, that is, it is invalid in the
/// * sense that its validity cannot be established. A conformant implementation
/// * shall support certRequestPermissions fields containing at least eight
/// * entries.
/// * It may be the case that an implementation of verification does not support
/// * the number of entries in  the certRequestPermissions field and the
/// * certRequestPermissions field is not relevant to the verification: this will
/// * occur, for example, if the certificate in question is the signing
/// * certificate for the SPDU and so the appPermissions field is relevant to
/// * the verification and the certRequestPermissions field is not. In this
/// * case, whether the implementation indicates that the signed SPDU is valid
/// * (because it could validate all relevant fields) or invalid (because it
/// * could not parse the entire certificate) is implementation-specific.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ToBeSignedCertificate {
    pub id: CertificateId,
    pub craca_id: HashedId3,
    pub crl_series: CrlSeries,
    pub validity_period: ValidityPeriod,
    pub region: Option<GeographicRegion>,
    pub assurance_level: Option<SubjectAssurance>,
    pub app_permissions: Option<SequenceOfPsidSsp>,
    pub cert_issue_permissions: Option<SequenceOfPsidGroupPermissions>,
    pub cert_request_permissions: Option<SequenceOfPsidGroupPermissions>,
    pub can_request_rollover: Option<()>,
    pub encryption_key: Option<PublicEncryptionKey>,
    pub verify_key_indicator: VerificationKeyIndicator,
    #[rasn(extension_addition, size("8"))]
    pub flags: Option<BitString>,
    #[rasn(extension_addition)]
    pub app_extensions: SequenceOfAppExtensions,
    #[rasn(extension_addition)]
    pub cert_issue_extensions: SequenceOfCertIssueExtensions,
    #[rasn(extension_addition)]
    pub cert_request_extension: SequenceOfCertRequestExtensions,
}

impl ToBeSignedCertificate {
    pub fn new(
        id: CertificateId,
        craca_id: HashedId3,
        crl_series: CrlSeries,
        validity_period: ValidityPeriod,
        region: Option<GeographicRegion>,
        assurance_level: Option<SubjectAssurance>,
        app_permissions: Option<SequenceOfPsidSsp>,
        cert_issue_permissions: Option<SequenceOfPsidGroupPermissions>,
        cert_request_permissions: Option<SequenceOfPsidGroupPermissions>,
        can_request_rollover: Option<()>,
        encryption_key: Option<PublicEncryptionKey>,
        verify_key_indicator: VerificationKeyIndicator,
        flags: Option<BitString>,
        app_extensions: SequenceOfAppExtensions,
        cert_issue_extensions: SequenceOfCertIssueExtensions,
        cert_request_extension: SequenceOfCertRequestExtensions,
    ) -> Self {
        Self {
            id,
            craca_id,
            crl_series,
            validity_period,
            region,
            assurance_level,
            app_permissions,
            cert_issue_permissions,
            cert_request_permissions,
            can_request_rollover,
            encryption_key,
            verify_key_indicator,
            flags,
            app_extensions,
            cert_issue_extensions,
            cert_request_extension,
        }
    }
}

///*
/// * @brief This structure contains the data to be hashed when generating or
/// * verifying a signature. See 6.3.4 for the specification of the input to the
/// * hash.
/// *
/// * @param payload: contains data that is provided by the entity that invokes
/// * the SDS.
/// *
/// * @param headerInfo: contains additional data that is inserted by the SDS.
/// * This structure is used as follows to determine the "data input" to the
/// * hash operation for signing or verification as specified in 5.3.1.2.2 or
/// * 5.3.1.3.
/// *   - If payload does not contain the field omitted, the data input to the
/// * hash operation is the COER encoding of the ToBeSignedData.
/// *   - If payload field in this ToBeSignedData instance contains the field
/// * omitted, the data input to the hash operation is the COER encoding of the
/// * ToBeSignedData, concatenated with the hash of the omitted payload. The hash
/// * of the omitted payload is calculated with the same hash algorithm that is
/// * used to calculate the hash of the data input for signing or verification.
/// * The data input to the hash operation is simply the COER enocding of the
/// * ToBeSignedData, concatenated with the hash of the omitted payload: there is
/// * no additional wrapping or length indication. As noted in 5.2.4.3.4, the
/// * means by which the signer and verifier establish the contents of the
/// * omitted payload are out of scope for this standard.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the SignedDataPayload if it is of type data, and to the
/// * HeaderInfo.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ToBeSignedData {
    pub payload: SignedDataPayload,
    pub header_info: HeaderInfo,
}

impl ToBeSignedData {
    pub fn new(payload: SignedDataPayload, header_info: HeaderInfo) -> Self {
        Self {
            payload,
            header_info,
        }
    }
}

///*
/// * @brief The contents of this field depend on whether the certificate is an
/// * implicit or an explicit certificate.
/// *
/// * @param verificationKey: is included in explicit certificates. It contains
/// * the public key to be used to verify signatures generated by the holder of
/// * the Certificate.
/// *
/// * @param reconstructionValue: is included in implicit certificates. It
/// * contains the reconstruction value, which is used to recover the public key
/// * as specified in SEC 4 and 5.3.2.
/// *
/// * @note Critical information fields: If present, this is a critical
/// * information field as defined in 5.2.5. An implementation that does not
/// * recognize the indicated CHOICE for this type when verifying a signed SPDU
/// * shall indicate that the signed SPDU is invalid indicate that the signed
/// * SPDU is invalid in the sense of 4.2.2.3.2, that is, it is invalid in the
/// * sense that its validity cannot be established.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the PublicVerificationKey and to the EccP256CurvePoint. The
/// * EccP256CurvePoint is encoded in compressed form, i.e., such that the
/// * choice indicated within the EccP256CurvePoint is compressed-y-0 or
/// * compressed-y-1.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum VerificationKeyIndicator {
    VerificationKey(PublicVerificationKey),
    ReconstructionValue(EccP256CurvePoint),
}

pub const CERT_EXT_ID__OPERATING_ORGANIZATION: ExtId = ExtId(1);

pub const ETSI_HEADER_INFO_CONTRIBUTOR_ID: HeaderInfoContributorId = HeaderInfoContributorId(2);

pub const IEEE1609HEADER_INFO_CONTRIBUTOR_ID: HeaderInfoContributorId = HeaderInfoContributorId(1);

pub const ISO21177EXTENDED_AUTH: PduFunctionalType = PduFunctionalType(2);

pub const ISO21177SESSION_EXTENSION: PduFunctionalType = PduFunctionalType(3);

pub const P2PCD8BYTE_LEARNING_REQUEST_ID: Ieee1609HeaderInfoExtensionId =
    Ieee1609HeaderInfoExtensionId(ExtId(1));

pub const TLS_HANDSHAKE: PduFunctionalType = PduFunctionalType(1);

// =====================================================
// Ieee1609Dot2BaseTypes
// { iso(1) identified-organization(3) ieee(111) standards-association-numbered-series-standards(2) wave-stds(1609) dot2(2) base(1) base-types(2) major-version-2(2) minor-version-4(4) }
// =====================================================

///*
/// * @brief This structure specifies the bytes of a public encryption key for
/// * a particular algorithm. Supported public key encryption algorithms are
/// * defined in 5.3.5.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2 if it appears in a
/// * HeaderInfo or in a ToBeSignedCertificate. See the definitions of HeaderInfo
/// * and ToBeSignedCertificate for a specification of the canonicalization
/// * operations.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum BasePublicEncryptionKey {
    EciesNistP256(EccP256CurvePoint),
    EciesBrainpoolP256r1(EccP256CurvePoint),
    #[rasn(extension_addition)]
    EcencSm2(EccP256CurvePoint),
}

///*
/// * @brief This structure represents a bitmap representation of a SSP. The
/// * mapping of the bits of the bitmap to constraints on the signed SPDU is
/// * PSID-specific.
/// *
/// * @note Consistency with issuing certificate: If a certificate has an
/// * appPermissions entry A for which the ssp field is bitmapSsp, A is
/// * consistent with the issuing certificate if the  certificate contains one
/// * of the following:
/// *   - (OPTION 1) A SubjectPermissions field indicating the choice all and
/// * no PsidSspRange field containing the psid field in A;
/// *   - (OPTION 2) A PsidSspRange P for which the following holds:
/// *     - The psid field in P is equal to the psid field in A and one of the
/// * following is true:
/// *       - EITHER The sspRange field in P indicates all
/// *       - OR The sspRange field in P indicates bitmapSspRange and for every
/// * bit set to 1 in the sspBitmask in P, the bit in the identical position in
/// * the sspValue in A is set equal to the bit in that position in the
/// * sspValue in P.
/// *
/// * @note A BitmapSsp B is consistent with a BitmapSspRange R if for every
/// * bit set to 1 in the sspBitmask in R, the bit in the identical position in
/// * B is set equal to the bit in that position in the sspValue in R. For each
/// * bit set to 0 in the sspBitmask in R, the corresponding bit in the
/// * identical position in B may be freely set to 0 or 1, i.e., if a bit is
/// * set to 0 in the sspBitmask in R, the value of corresponding bit in the
/// * identical position in B has no bearing on whether B and R are consistent.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("0..=31"))]
pub struct BitmapSsp(pub OctetString);

///*
/// * @brief This structure represents a bitmap representation of a SSP. The
/// * sspValue indicates permissions. The sspBitmask contains an octet string
/// * used to permit or constrain sspValue fields in issued certificates. The
/// * sspValue and sspBitmask fields shall be of the same length.
/// *
/// * @note Consistency with issuing certificate: If a certificate has an
/// * PsidSspRange value P for which the sspRange field is bitmapSspRange,
/// * P is consistent with the issuing certificate if the issuing certificate
/// * contains one of the following:
/// *   - (OPTION 1) A SubjectPermissions field indicating the choice all and
/// * no PsidSspRange field containing the psid field in P;
/// *   - (OPTION 2) A PsidSspRange R for which the following holds:
/// *     - The psid field in R is equal to the psid field in P and one of the
/// * following is true:
/// *       - EITHER The sspRange field in R indicates all
/// *       - OR The sspRange field in R indicates bitmapSspRange and for every
/// * bit set to 1 in the sspBitmask in R:
/// *         - The bit in the identical position in the sspBitmask in P is set
/// * equal to 1, AND
/// *         - The bit in the identical position in the sspValue in P is set equal
/// * to the bit in that position in the sspValue in R.
/// *
/// * Reference ETSI TS 103 097 for more information on bitmask SSPs.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct BitmapSspRange {
    #[rasn(size("1..=32"))]
    pub ssp_value: OctetString,
    #[rasn(size("1..=32"))]
    pub ssp_bitmask: OctetString,
}

impl BitmapSspRange {
    pub fn new(ssp_value: OctetString, ssp_bitmask: OctetString) -> Self {
        Self {
            ssp_value,
            ssp_bitmask,
        }
    }
}

///*
/// * @brief This structure specifies a circle with its center at center, its
/// * radius given in meters, and located tangential to the reference ellipsoid.
/// * The indicated region is all the points on the surface of the reference
/// * ellipsoid whose distance to the center point over the reference ellipsoid
/// * is less than or equal to the radius. A point which contains an elevation
/// * component is considered to be within the circular region if its horizontal
/// * projection onto the reference ellipsoid lies within the region.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CircularRegion {
    pub center: TwoDLocation,
    pub radius: Uint16,
}

impl CircularRegion {
    pub fn new(center: TwoDLocation, radius: Uint16) -> Self {
        Self { center, radius }
    }
}

///*
/// * @brief A conformant implementation that supports CountryAndRegions shall
/// * support a regions field containing at least eight entries.
/// * A conformant implementation that implements this type shall recognize
/// * (in the sense of "be able to determine whether a two dimensional location
/// * lies inside or outside the borders identified by") at least one value of
/// * UnCountryId and at least one value for a region within the country
/// * indicated by that recognized UnCountryId value. In this version of this
/// * standard, the only means to satisfy this is for a conformant
/// * implementation to recognize the value of UnCountryId indicating USA and
/// * at least one of the FIPS state codes for US states. The Protocol
/// * Implementation Conformance Statement (PICS) provided in Annex A allows
/// * an implementation to state which UnCountryId values it recognizes and
/// * which region values are recognized within that country.
/// * If a verifying implementation is required to check that an relevant
/// * geographic information in a signed SPDU is consistent with a certificate
/// * containing one or more instances of this type, then the SDS is permitted
/// * to indicate that the signed SPDU is valid even if some values of country
/// * or within regions are unrecognized in the sense defined above, so long
/// * as the recognized instances of this type completely contain the relevant
/// * geographic information. Informally, if the recognized values in the
/// * certificate allow the SDS to determine that the SPDU is valid, then it
/// * can make that determination even if there are also unrecognized values
/// * in the certificate. This field is therefore not a "critical information
/// * field" as defined in 5.2.6, because unrecognized values are permitted so
/// * long as the validity of the SPDU can be established with the recognized
/// * values. However, as discussed in 5.2.6, the presence of an unrecognized
/// * value in a certificate can make it impossible to determine whether the
/// * certificate is valid and so whether the SPDU is valid.
/// * In this type:
/// *
/// * @param countryOnly: is a UnCountryId as defined above.
/// *
/// * @param regions: identifies one or more regions within the country. If
/// * country indicates the United States of America, the values in this field
/// * identify the state or statistically equivalent entity using the integer
/// * version of the 2010 FIPS codes as provided by the U.S. Census Bureau
/// * (see normative references in Clause 0). For other values of country, the
/// * meaning of region is not defined in this version of this standard.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CountryAndRegions {
    pub country_only: UnCountryId,
    pub regions: SequenceOfUint8,
}

impl CountryAndRegions {
    pub fn new(country_only: UnCountryId, regions: SequenceOfUint8) -> Self {
        Self {
            country_only,
            regions,
        }
    }
}

///*
/// * @brief A conformant implementation that supports CountryAndSubregions
/// * shall support a regionAndSubregions field containing at least eight
/// * entries.
/// * A conformant implementation that implements this type shall recognize
/// * (in the sense of be able to determine whether a two dimensional location
/// * lies inside or outside the borders identified by) at least one value of
/// * country and at least one value for a region within the country indicated
/// * by that recognized country value. In this version of this standard, the
/// * only means to satisfy this is for a conformant implementation to recognize
/// * the value of UnCountryId indicating USA and at least one of the FIPS state
/// * codes for US states. The Protocol Implementation Conformance Statement
/// * (PICS) provided in Annex A allows an implementation to state which
/// * UnCountryId values it recognizes and which region values are recognized
/// * within that country.
/// * If a verifying implementation is required to check that an relevant
/// * geographic information in a signed SPDU is consistent with a certificate
/// * containing one or more instances of this type, then the SDS is permitted
/// * to indicate that the signed SPDU is valid even if some values of country
/// * or within regionAndSubregions are unrecognized in the sense defined above,
/// * so long as the recognized instances of this type completely contain the
/// * relevant geographic information. Informally, if the recognized values in
/// * the certificate allow the SDS to determine that the SPDU is valid, then
/// * it can make that determination even if there are also unrecognized values
/// * in the certificate. This field is therefore not a "critical information
/// * field" as defined in 5.2.6, because unrecognized values are permitted so
/// * long as the validity of the SPDU can be established with the recognized
/// * values. However, as discussed in 5.2.6, the presence of an unrecognized
/// * value in a certificate can make it impossible to determine whether the
/// * certificate is valid and so whether the SPDU is valid.
/// * In this structure:
/// *
/// * @param countryOnly: is a UnCountryId as defined above.
/// *
/// * @param regionAndSubregions: identifies one or more subregions within
/// * country.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CountryAndSubregions {
    pub country_only: UnCountryId,
    pub region_and_subregions: SequenceOfRegionAndSubregions,
}

impl CountryAndSubregions {
    pub fn new(
        country_only: UnCountryId,
        region_and_subregions: SequenceOfRegionAndSubregions,
    ) -> Self {
        Self {
            country_only,
            region_and_subregions,
        }
    }
}

///*
/// * @brief This type is defined only for backwards compatibility.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct CountryOnly(pub UnCountryId);

///*
/// * @brief This integer identifies a series of CRLs issued under the authority
/// * of a particular CRACA.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct CrlSeries(pub Uint16);

///*
/// * @brief This structure represents the duration of validity of a
/// * certificate. The Uint16 value is the duration, given in the units denoted
/// * by the indicated choice. A year is considered to be 31556952 seconds,
/// * which is the average number of seconds in a year.
/// *
/// * @note Years can be mapped more closely to wall-clock days using the hours
/// * choice for up to 7 years and the sixtyHours choice for up to 448 years.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Duration {
    Microseconds(Uint16),
    Milliseconds(Uint16),
    Seconds(Uint16),
    Minutes(Uint16),
    Hours(Uint16),
    SixtyHours(Uint16),
    Years(Uint16),
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EccP256CurvePointUncompressedP256 {
    #[rasn(size("32"))]
    pub x: OctetString,
    #[rasn(size("32"))]
    pub y: OctetString,
}

impl EccP256CurvePointUncompressedP256 {
    pub fn new(x: OctetString, y: OctetString) -> Self {
        Self { x, y }
    }
}

///*
/// * @brief This structure specifies a point on an elliptic curve in Weierstrass
/// * form defined over a 256-bit prime number. The curves supported in this
/// * standard are NIST p256 as defined in FIPS 186-4, Brainpool p256r1 as
/// * defined in RFC 5639, and the SM2 curve as defined in GB/T 32918.5-2017.
/// * The fields in this structure are OCTET STRINGS produced with the elliptic
/// * curve point encoding and decoding methods defined in subclause 5.5.6 of
/// * IEEE Std 1363-2000. The x-coordinate is encoded as an unsigned integer of
/// * length 32 octets in network byte order for all values of the CHOICE; the
/// * encoding of the y-coordinate y depends on whether the point is x-only,
/// * compressed, or uncompressed. If the point is x-only, y is omitted. If the
/// * point is compressed, the value of type depends on the least significant
/// * bit of y: if the least significant bit of y is 0, type takes the value
/// * compressed-y-0, and if the least significant bit of y is 1, type takes the
/// * value compressed-y-1. If the point is uncompressed, y is encoded explicitly
/// * as an unsigned integer of length 32 octets in network byte order.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2 if it appears in a
/// * HeaderInfo or in a ToBeSignedCertificate. See the definitions of HeaderInfo
/// * and ToBeSignedCertificate for a specification of the canonicalization
/// * operations.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum EccP256CurvePoint {
    #[rasn(size("32"))]
    XOnly(OctetString),
    Fill(()),
    #[rasn(size("32"))]
    CompressedY0(OctetString),
    #[rasn(size("32"))]
    CompressedY1(OctetString),
    UncompressedP256(EccP256CurvePointUncompressedP256),
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EccP384CurvePointUncompressedP384 {
    #[rasn(size("48"))]
    pub x: OctetString,
    #[rasn(size("48"))]
    pub y: OctetString,
}

impl EccP384CurvePointUncompressedP384 {
    pub fn new(x: OctetString, y: OctetString) -> Self {
        Self { x, y }
    }
}

///*
/// * @brief This structure specifies a point on an elliptic curve in
/// * Weierstrass form defined over a 384-bit prime number. The only supported
/// * such curve in this standard is Brainpool p384r1 as defined in RFC 5639.
/// * The fields in this structure are octet strings produced with the elliptic
/// * curve point encoding and decoding methods defined in subclause 5.5.6 of
/// * IEEE Std 1363-2000. The x-coordinate is encoded as an unsigned integer of
/// * length 48 octets in network byte order for all values of the CHOICE; the
/// * encoding of the y-coordinate y depends on whether the point is x-only,
/// * compressed, or uncompressed. If the point is x-only, y is omitted. If the
/// * point is compressed, the value of type depends on the least significant
/// * bit of y: if the least significant bit of y is 0, type takes the value
/// * compressed-y-0, and if the least significant bit of y is 1, type takes the
/// * value compressed-y-1. If the point is uncompressed, y is encoded
/// * explicitly as an unsigned integer of length 48 octets in network byte order.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2 if it appears in a
/// * HeaderInfo or in a ToBeSignedCertificate. See the definitions of HeaderInfo
/// * and ToBeSignedCertificate for a specification of the canonicalization
/// * operations.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum EccP384CurvePoint {
    #[rasn(size("48"))]
    XOnly(OctetString),
    Fill(()),
    #[rasn(size("48"))]
    CompressedY0(OctetString),
    #[rasn(size("48"))]
    CompressedY1(OctetString),
    UncompressedP384(EccP384CurvePointUncompressedP384),
}

///*
/// * @brief This structure represents an ECDSA signature. The signature is
/// * generated as specified in 5.3.1.
/// *
/// * If the signature process followed the specification of FIPS 186-4
/// * and output the integer r, r is represented as an EccP256CurvePoint
/// * indicating the selection x-only.
/// *
/// * If the signature process followed the specification of SEC 1 and
/// * output the elliptic curve point R to allow for fast verification, R is
/// * represented as an EccP256CurvePoint indicating the choice compressed-y-0,
/// * compressed-y-1, or uncompressed at the sender's discretion.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. When this data structure
/// * is canonicalized, the EccP256CurvePoint in rSig is represented in the
/// * form x-only.
/// *
/// * @note When the signature is of form x-only, the x-value in rSig is
/// * an integer mod n, the order of the group; when the signature is of form
/// * compressed-y-\*, the x-value in rSig is an integer mod p, the underlying
/// * prime defining the finite field. In principle this means that to convert a
/// * signature from form compressed-y-\* to form x-only, the converter checks
/// * the x-value to see if it lies between n and p and reduces it mod n if so.
/// * In practice this check is unnecessary: Haase's Theorem states that
/// * difference between n and p is always less than 2*square-root(p), and so the
/// * chance that an integer lies between n and p, for a 256-bit curve, is
/// * bounded above by approximately square-root(p)/p or 2^(-128). For the
/// * 256-bit curves in this standard, the exact values of n and p in hexadecimal
/// * are:
/// *
/// * NISTp256:
/// *   - p = FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF
/// *   - n = FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
/// *
/// * Brainpoolp256:
/// *   - p = A9FB57DBA1EEA9BC3E660A909D838D726E3BF623D52620282013481D1F6E5377
/// *   - n = A9FB57DBA1EEA9BC3E660A909D838D718C397AA3B561A6F7901E0E82974856A7
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EcdsaP256Signature {
    pub r_sig: EccP256CurvePoint,
    #[rasn(size("32"))]
    pub s_sig: OctetString,
}

impl EcdsaP256Signature {
    pub fn new(r_sig: EccP256CurvePoint, s_sig: OctetString) -> Self {
        Self { r_sig, s_sig }
    }
}

///*
/// * @brief This structure represents an ECDSA signature. The signature is
/// * generated as specified in 5.3.1.
/// *
/// * If the signature process followed the specification of FIPS 186-4
/// * and output the integer r, r is represented as an EccP384CurvePoint
/// * indicating the selection x-only.
/// *
/// * If the signature process followed the specification of SEC 1 and
/// * output the elliptic curve point R to allow for fast verification, R is
/// * represented as an EccP384CurvePoint indicating the choice compressed-y-0,
/// * compressed-y-1, or uncompressed at the sender's discretion.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. When this data structure
/// * is canonicalized, the EccP384CurvePoint in rSig is represented in the
/// * form x-only.
/// *
/// * @note When the signature is of form x-only, the x-value in rSig is
/// * an integer mod n, the order of the group; when the signature is of form
/// * compressed-y-\*, the x-value in rSig is an integer mod p, the underlying
/// * prime defining the finite field. In principle this means that to convert a
/// * signature from form compressed-y-* to form x-only, the converter checks the
/// * x-value to see if it lies between n and p and reduces it mod n if so. In
/// * practice this check is unnecessary: Haase's Theorem states that difference
/// * between n and p is always less than 2*square-root(p), and so the chance
/// * that an integer lies between n and p, for a 384-bit curve, is bounded
/// * above by approximately square-root(p)/p or 2^(-192). For the 384-bit curve
/// * in this standard, the exact values of n and p in hexadecimal are:
/// *   - p = 8CB91E82A3386D280F5D6F7E50E641DF152F7109ED5456B412B1DA197FB71123
/// * ACD3A729901D1A71874700133107EC53
/// *   - n = 8CB91E82A3386D280F5D6F7E50E641DF152F7109ED5456B31F166E6CAC0425A7
/// * CF3AB6AF6B7FC3103B883202E9046565
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EcdsaP384Signature {
    pub r_sig: EccP384CurvePoint,
    #[rasn(size("48"))]
    pub s_sig: OctetString,
}

impl EcdsaP384Signature {
    pub fn new(r_sig: EccP384CurvePoint, s_sig: OctetString) -> Self {
        Self { r_sig, s_sig }
    }
}

///*
/// * @brief This data structure is used to transfer a 16-byte symmetric key
/// * encrypted using SM2 encryption as specified in 5.3.3. The symmetric key is
/// * input to the key encryption process with no headers, encapsulation, or
/// * length indication. Encryption and decryption are carried out as specified
/// * in 5.3.5.2.
/// *
/// * @param v: is the sender's ephemeral public key, which is the output V from
/// * encryption as specified in 5.3.5.2.
/// *
/// * @param c: is the encrypted symmetric key, which is the output C from
/// * encryption as specified in 5.3.5.2. The algorithm for the symmetric key
/// * is identified by the CHOICE indicated in the following SymmetricCiphertext.
/// * For SM2 this algorithm shall be SM4.
/// *
/// * @param t: is the authentication tag, which is the output tag from
/// * encryption as specified in 5.3.5.2.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EcencP256EncryptedKey {
    pub v: EccP256CurvePoint,
    #[rasn(size("16"))]
    pub c: OctetString,
    #[rasn(size("32"))]
    pub t: OctetString,
}

impl EcencP256EncryptedKey {
    pub fn new(v: EccP256CurvePoint, c: OctetString, t: OctetString) -> Self {
        Self { v, c, t }
    }
}

///*
/// * @brief This data structure is used to transfer a 16-byte symmetric key
/// * encrypted using ECIES as specified in IEEE Std 1363a-2004. The symmetric
/// * key is input to the key encryption process with no headers, encapsulation,
/// * or length indication. Encryption and decryption are carried out as
/// * specified in 5.3.5.1.
/// *
/// * @param v: is the sender's ephemeral public key, which is the output V from
/// * encryption as specified in 5.3.5.1.
/// *
/// * @param c: is the encrypted symmetric key, which is the output C from
/// * encryption as specified in 5.3.5.1. The algorithm for the symmetric key
/// * is identified by the CHOICE indicated in the following SymmetricCiphertext.
/// * For ECIES this shall be AES-128.
/// *
/// * @param t: is the authentication tag, which is the output tag from
/// * encryption as specified in 5.3.5.1.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EciesP256EncryptedKey {
    pub v: EccP256CurvePoint,
    #[rasn(size("16"))]
    pub c: OctetString,
    #[rasn(size("16"))]
    pub t: OctetString,
}

impl EciesP256EncryptedKey {
    pub fn new(v: EccP256CurvePoint, c: OctetString, t: OctetString) -> Self {
        Self { v, c, t }
    }
}

///*
/// * @brief This structure represents a elliptic curve signature where the
/// * component r is constrained to be an integer. This structure supports SM2
/// * signatures as specified in 5.3.1.3.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EcsigP256Signature {
    #[rasn(size("32"))]
    pub r_sig: OctetString,
    #[rasn(size("32"))]
    pub s_sig: OctetString,
}

impl EcsigP256Signature {
    pub fn new(r_sig: OctetString, s_sig: OctetString) -> Self {
        Self { r_sig, s_sig }
    }
}

///*
/// * @brief This structure contains an estimate of the geodetic altitude above
/// * or below the WGS84 ellipsoid. The 16-bit value is interpreted as an
/// * integer number of decimeters representing the height above a minimum
/// * height of -409.5 m, with the maximum height being 6143.9 m.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Elevation(pub Uint16);

///*
/// * @brief This structure contains an encryption key, which may be a public or
/// * a symmetric key.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2 if it appears in a
/// * HeaderInfo or in a ToBeSignedCertificate. The canonicalization applies to
/// * the PublicEncryptionKey. See the definitions of HeaderInfo and
/// * ToBeSignedCertificate for a specification of the canonicalization
/// * operations.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum EncryptionKey {
    Public(PublicEncryptionKey),
    Symmetric(SymmetricEncryptionKey),
}

///*
/// * @brief This type is used as an identifier for instances of ExtContent
/// * within an EXT-TYPE.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct ExtId(pub u8);

///*
/// * @brief This parameterized type represents a (id, content) pair drawn from
/// * the set ExtensionTypes, which is constrained to contain objects defined by
/// * the class EXT-TYPE.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Extension {
    pub id: ExtId,
    pub content: Any,
}

impl Extension {
    pub fn new(id: ExtId, content: Any) -> Self {
        Self { id, content }
    }
}

///***************************************************************************
///                           Location Structures                             
///***************************************************************************
///*
/// * @brief This structure represents a geographic region of a specified form.
/// * A certificate is not valid if any part of the region indicated in its
/// * scope field lies outside the region indicated in the scope of its issuer.
/// *
/// * @param circularRegion: contains a single instance of the CircularRegion
/// * structure.
/// *
/// * @param rectangularRegion: is an array of RectangularRegion structures
/// * containing at least one entry. This field is interpreted as a series of
/// * rectangles, which may overlap or be disjoint. The permitted region is any
/// * point within any of the rectangles.
/// *
/// * @param polygonalRegion: contains a single instance of the PolygonalRegion
/// * structure.
/// *
/// * @param identifiedRegion: is an array of IdentifiedRegion structures
/// * containing at least one entry. The permitted region is any point within
/// * any of the identified regions.
/// *
/// * @note Critical information fields:
/// *   - If present, this is a critical information field as defined in 5.2.6.
/// * An implementation that does not recognize the indicated CHOICE when
/// * verifying a signed SPDU shall indicate that the signed SPDU is invalid in
/// * the sense of 4.2.2.3.2, that is, it is invalid in the sense that its
/// * validity cannot be established.
/// *   - If selected, rectangularRegion is a critical information field as
/// * defined in 5.2.6. An implementation that does not support the number of
/// * RectangularRegion in rectangularRegions when verifying a signed SPDU shall
/// * indicate that the signed SPDU is invalid in the sense of 4.2.2.3.2, that
/// * is, it is invalid in the sense that its validity cannot be established.
/// * A conformant implementation shall support rectangularRegions fields
/// * containing at least eight entries.
/// *   - If selected, identifiedRegion is a critical information field as
/// * defined in 5.2.6. An implementation that does not support the number of
/// * IdentifiedRegion in identifiedRegion shall reject the signed SPDU as
/// * invalid in the sense of 4.2.2.3.2, that is, it is invalid in the sense
/// * that its validity cannot be established. A conformant implementation shall
/// * support identifiedRegion fields containing at least eight entries.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum GeographicRegion {
    CircularRegion(CircularRegion),
    RectangularRegion(SequenceOfRectangularRegion),
    PolygonalRegion(PolygonalRegion),
    IdentifiedRegion(SequenceOfIdentifiedRegion),
}

///*
/// * @brief This is the group linkage value. See 5.1.3 and 7.3 for details of
/// * use.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct GroupLinkageValue {
    #[rasn(size("4"))]
    pub j_value: OctetString,
    #[rasn(size("9"))]
    pub value: OctetString,
}

impl GroupLinkageValue {
    pub fn new(j_value: OctetString, value: OctetString) -> Self {
        Self { j_value, value }
    }
}

///*
/// * @brief This structure identifies a hash algorithm. The value sha256,
/// * indicates SHA-256. The value sha384 indicates SHA-384. The value sm3
/// * indicates SM3. See 5.3.3 for more details.
/// *
/// * @note Critical information fields: This is a critical information field as
/// * defined in 5.2.6. An implementation that does not recognize the enumerated
/// * value of this type in a signed SPDU when verifying a signed SPDU shall
/// * indicate that the signed SPDU is invalid in the sense of 4.2.2.3.2, that
/// * is, it is invalid in the sense that its validity cannot be established.
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum HashAlgorithm {
    Sha256 = 0,
    #[rasn(extension_addition)]
    Sha384 = 1,
    #[rasn(extension_addition)]
    Sm3 = 2,
}

///*
/// * @brief This type contains the truncated hash of another data structure.
/// * The HashedId10 for a given data structure is calculated by calculating the
/// * hash of the encoded data structure and taking the low-order ten bytes of
/// * the hash output. The low-order ten bytes are the last ten bytes of the
/// * hash when represented in network byte order. If the data structure
/// * is subject to canonicalization it is canonicalized before hashing. See
/// * Example below.
/// *
/// * The hash algorithm to be used to calculate a HashedId10 within a
/// * structure depends on the context. In this standard, for each structure
/// * that includes a HashedId10 field, the corresponding text indicates how the
/// * hash algorithm is determined. See also the discussion in 5.3.9.
/// *
/// * Example: Consider the SHA-256 hash of the empty string:
/// *
/// * SHA-256("") =
/// * e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
/// *
/// * The HashedId10 derived from this hash corresponds to the following:
/// *
/// * HashedId10 = 934ca495991b7852b855.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("10"))]
pub struct HashedId10(pub OctetString);

///*
/// * @brief This type contains the truncated hash of another data structure.
/// * The HashedId3 for a given data structure is calculated by calculating the
/// * hash of the encoded data structure and taking the low-order three bytes of
/// * the hash output. The low-order three bytes are the last three bytes of the
/// * 32-byte hash when represented in network byte order. If the data structure
/// * is subject to canonicalization it is canonicalized before hashing. See
/// * Example below.
/// *
/// * The hash algorithm to be used to calculate a HashedId3 within a
/// * structure depends on the context. In this standard, for each structure
/// * that includes a HashedId3 field, the corresponding text indicates how the
/// * hash algorithm is determined. See also the discussion in 5.3.9.
/// *
/// * Example: Consider the SHA-256 hash of the empty string:
/// *
/// * SHA-256("") =
/// * e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
/// *
/// * The HashedId3 derived from this hash corresponds to the following:
/// *
/// * HashedId3 = 52b855.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("3"))]
pub struct HashedId3(pub OctetString);

///*
/// * @brief This data structure contains the truncated hash of another data
/// * structure. The HashedId32 for a given data structure is calculated by
/// * calculating the hash of the encoded data structure and taking the
/// * low-order 32 bytes of the hash output. The low-order 32 bytes are the last
/// * 32 bytes of the hash when represented in network byte order. If the data
/// * structure is subject to canonicalization it is canonicalized before
/// * hashing. See Example below.
/// *
/// * The hash algorithm to be used to calculate a HashedId32 within a
/// * structure depends on the context. In this standard, for each structure
/// * that includes a HashedId32 field, the corresponding text indicates how the
/// * hash algorithm is determined. See also the discussion in 5.3.9.
/// *
/// * Example: Consider the SHA-256 hash of the empty string:
/// *
/// * SHA-256("") =
/// * e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
/// *
/// * The HashedId32 derived from this hash corresponds to the following:
/// *
/// * HashedId32 = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8
/// * 55.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("32"))]
pub struct HashedId32(pub OctetString);

///*
/// * @brief This data structure contains the truncated hash of another data
/// * structure. The HashedId48 for a given data structure is calculated by
/// * calculating the hash of the encoded data structure and taking the
/// * low-order 48 bytes of the hash output. The low-order 48 bytes are the last
/// * 48 bytes of the hash when represented in network byte order. If the data
/// * structure is subject to canonicalization it is canonicalized before
/// * hashing. See Example below.
/// *
/// * The hash algorithm to be used to calculate a HashedId48 within a
/// * structure depends on the context. In this standard, for each structure
/// * that includes a HashedId48 field, the corresponding text indicates how the
/// * hash algorithm is determined. See also the discussion in 5.3.9.
/// *
/// * Example: Consider the SHA-384 hash of the empty string:
/// *
/// * SHA-384("") = 38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6
/// * e1da274edebfe76f65fbd51ad2f14898b95b
/// *
/// * The HashedId48 derived from this hash corresponds to the following:
/// *
/// * HashedId48 = 38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e
/// * 1da274edebfe76f65fbd51ad2f14898b95b.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("48"))]
pub struct HashedId48(pub OctetString);

///*
/// * @brief This type contains the truncated hash of another data structure.
/// * The HashedId8 for a given data structure is calculated by calculating the
/// * hash of the encoded data structure and taking the low-order eight bytes of
/// * the hash output. The low-order eight bytes are the last eight bytes of the
/// * hash when represented in network byte order. If the data structure
/// * is subject to canonicalization it is canonicalized before hashing. See
/// * Example below.
/// *
/// * The hash algorithm to be used to calculate a HashedId8 within a
/// * structure depends on the context. In this standard, for each structure
/// * that includes a HashedId8 field, the corresponding text indicates how the
/// * hash algorithm is determined. See also the discussion in 5.3.9.
/// *
/// * Example: Consider the SHA-256 hash of the empty string:
/// *
/// * SHA-256("") =
/// * e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
/// *
/// * The HashedId8 derived from this hash corresponds to the following:
/// *
/// * HashedId8 = a495991b7852b855.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct HashedId8(pub OctetString);

///*
/// * @brief This is a UTF-8 string as defined in IETF RFC 3629. The contents
/// * are determined by policy.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("0..=255"))]
pub struct Hostname(pub Utf8String);

///***************************************************************************
///                             Pseudonym Linkage                             
///***************************************************************************
///*
/// * @brief This atomic type is used in the definition of other data structures.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct IValue(pub Uint16);

///*
/// * @brief This structure indicates the region of validity of a certificate
/// * using region identifiers.
/// * A conformant implementation that supports this type shall support at least
/// * one of the possible CHOICE values. The Protocol Implementation Conformance
/// * Statement (PICS) provided in Annex A allows an implementation to state
/// * which CountryOnly values it recognizes.
/// *
/// * @param countryOnly: indicates that only a country (or a geographic entity
/// * included in a country list) is given.
/// *
/// * @param countryAndRegions: indicates that one or more top-level regions
/// * within a country (as defined by the region listing associated with that
/// * country) is given.
/// *
/// * @param countryAndSubregions: indicates that one or more regions smaller
/// * than the top-level regions within a country (as defined by the region
/// * listing associated with that country) is given.
/// *
/// * Critical information fields: If present, this is a critical
/// * information field as defined in 5.2.6. An implementation that does not
/// * recognize the indicated CHOICE when verifying a signed SPDU shall indicate
/// * that the signed SPDU is invalid in the sense of 4.2.2.3.2, that is, it is
/// * invalid in the sense that its validity cannot be established.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum IdentifiedRegion {
    CountryOnly(UnCountryId),
    CountryAndRegions(CountryAndRegions),
    CountryAndSubregions(CountryAndSubregions),
}

///*
/// * @brief The known latitudes are from -900,000,000 to +900,000,000 in 0.1
/// * microdegree intervals.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("-900000000..=900000000"))]
pub struct KnownLatitude(pub NinetyDegreeInt);

///*
/// * @brief The known longitudes are from -1,799,999,999 to +1,800,000,000 in
/// * 0.1 microdegree intervals.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("-1799999999..=1800000000"))]
pub struct KnownLongitude(pub OneEightyDegreeInt);

///*
/// * @brief This structure contains a LA Identifier for use in the algorithms
/// * specified in 5.1.3.4.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("2"))]
pub struct LaId(pub OctetString);

///*
/// * @brief This type contains an INTEGER encoding an estimate of the latitude
/// * with precision 1/10th microdegree relative to the World Geodetic System
/// * (WGS)-84 datum as defined in NIMA Technical Report TR8350.2.
/// * The integer in the latitude field is no more than 900 000 000 and no less
/// * than ?900 000 000, except that the value 900 000 001 is used to indicate
/// * the latitude was not available to the sender.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Latitude(pub NinetyDegreeInt);

///*
/// * @brief This structure contains a linkage seed value for use in the
/// * algorithms specified in 5.1.3.4.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct LinkageSeed(pub OctetString);

///*
/// * @brief This is the individual linkage value. See 5.1.3 and 7.3 for details
/// * of use.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("9"))]
pub struct LinkageValue(pub OctetString);

///*
/// * @brief This type contains an INTEGER encoding an estimate of the longitude
/// * with precision 1/10th microdegree relative to the World Geodetic System
/// * (WGS)-84 datum as defined in NIMA Technical Report TR8350.2.
/// * The integer in the longitude field is no more than 1 800 000 000 and no
/// * less than ?1 799 999 999, except that the value 1 800 000 001 is used to
/// * indicate that the longitude was not available to the sender.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Longitude(pub OneEightyDegreeInt);

///*
/// * @brief The integer in the latitude field is no more than 900,000,000 and
/// * no less than -900,000,000, except that the value 900,000,001 is used to
/// * indicate the latitude was not available to the sender.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-900000000..=900000001"))]
pub struct NinetyDegreeInt(pub i32);

///*
/// * @brief The integer in the longitude field is no more than 1,800,000,000
/// * and no less than -1,799,999,999, except that the value 1,800,000,001 is
/// * used to indicate that the longitude was not available to the sender.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1799999999..=1800000001"))]
pub struct OneEightyDegreeInt(pub i32);

///***************************************************************************
///                            OCTET STRING Types                             
///***************************************************************************
///*
/// * @brief This is a synonym for ASN.1 OCTET STRING, and is used in the
/// * definition of other data structures.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Opaque(pub OctetString);

///*
/// * @brief This structure defines a region using a series of distinct
/// * geographic points, defined on the surface of the reference ellipsoid. The
/// * region is specified by connecting the points in the order they appear,
/// * with each pair of points connected by the geodesic on the reference
/// * ellipsoid. The polygon is completed by connecting the final point to the
/// * first point. The allowed region is the interior of the polygon and its
/// * boundary.
/// *
/// * A point which contains an elevation component is considered to be
/// * within the polygonal region if its horizontal projection onto the
/// * reference ellipsoid lies within the region.
/// *
/// * A valid PolygonalRegion contains at least three points. In a valid
/// * PolygonalRegion, the implied lines that make up the sides of the polygon
/// * do not intersect.
/// *
/// * @note This type does not support enclaves / exclaves. This might be
/// * addressed in a future version of this standard.
/// *
/// * @note Critical information fields: If present, this is a critical
/// * information field as defined in 5.2.6. An implementation that does not
/// * support the number of TwoDLocation in the PolygonalRegion when verifying a
/// * signed SPDU shall indicate that the signed SPDU is invalid. A compliant
/// * implementation shall support PolygonalRegions containing at least eight
/// * TwoDLocation entries.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("3.."))]
pub struct PolygonalRegion(pub SequenceOf<TwoDLocation>);

///*
/// * @brief This type represents the PSID defined in IEEE Std 1609.12.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0.."))]
pub struct Psid(pub Integer);

///***************************************************************************
///                              PSID / ITS-AID                               
///***************************************************************************
///*
/// * @brief This structure represents the permissions that the certificate
/// * holder has with respect to activities for a single application area,
/// * identified by a Psid.
/// *
/// * @note The determination as to whether the activities are consistent with
/// * the permissions indicated by the PSID and ServiceSpecificPermissions is
/// * made by the SDEE and not by the SDS; the SDS provides the PSID and SSP
/// * information to the SDEE to enable the SDEE to make that determination.
/// * See 5.2.4.3.3 for more information.
/// *
/// * @note The SDEE specification is expected to specify what application
/// * activities are permitted by particular ServiceSpecificPermissions values.
/// * The SDEE specification is also expected EITHER to specify application
/// * activities that are permitted if the ServiceSpecificPermissions is
/// * omitted, OR to state that the ServiceSpecificPermissions need to always be
/// * present.
/// *
/// * @note Consistency with signed SPDU: As noted in 5.1.1,
/// * consistency between the SSP and the signed SPDU is defined by rules
/// * specific to the given PSID and is out of scope for this standard.
/// *
/// * @note Consistency with issuing certificate: If a certificate has an
/// * appPermissions entry A for which the ssp field is omitted, A is consistent
/// * with the issuing certificate if the issuing certificate contains a
/// * PsidSspRange P for which the following holds:
/// *   - The psid field in P is equal to the psid field in A and one of the
/// * following is true:
/// *     - The sspRange field in P indicates all.
/// *     - The sspRange field in P indicates opaque and one of the entries in
/// * opaque is an OCTET STRING of length 0.
/// *
/// * For consistency rules for other forms of the ssp field, see the
/// * following subclauses.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PsidSsp {
    pub psid: Psid,
    pub ssp: Option<ServiceSpecificPermissions>,
}

impl PsidSsp {
    pub fn new(psid: Psid, ssp: Option<ServiceSpecificPermissions>) -> Self {
        Self { psid, ssp }
    }
}

///*
/// * @brief This structure represents the certificate issuing or requesting
/// * permissions of the certificate holder with respect to one particular set
/// * of application permissions.
/// *
/// * @param psid: identifies the application area.
/// *
/// * @param sspRange: identifies the SSPs associated with that PSID for which
/// * the holder may issue or request certificates. If sspRange is omitted, the
/// * holder may issue or request certificates for any SSP for that PSID.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PsidSspRange {
    pub psid: Psid,
    pub ssp_range: Option<SspRange>,
}

impl PsidSspRange {
    pub fn new(psid: Psid, ssp_range: Option<SspRange>) -> Self {
        Self { psid, ssp_range }
    }
}

///*
/// * @brief This structure specifies a public encryption key and the associated
/// * symmetric algorithm which is used for bulk data encryption when encrypting
/// * for that public key.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2 if it appears in a
/// * HeaderInfo or in a ToBeSignedCertificate. The canonicalization applies to
/// * the BasePublicEncryptionKey. See the definitions of HeaderInfo and
/// * ToBeSignedCertificate for a specification of the canonicalization
/// * operations.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PublicEncryptionKey {
    pub supported_symm_alg: SymmAlgorithm,
    pub public_key: BasePublicEncryptionKey,
}

impl PublicEncryptionKey {
    pub fn new(supported_symm_alg: SymmAlgorithm, public_key: BasePublicEncryptionKey) -> Self {
        Self {
            supported_symm_alg,
            public_key,
        }
    }
}

///*
/// * @brief This structure represents a public key and states with what
/// * algorithm the public key is to be used. Cryptographic mechanisms are
/// * defined in 5.3.
/// * An EccP256CurvePoint or EccP384CurvePoint within a PublicVerificationKey
/// * structure is invalid if it indicates the choice x-only.
/// *
/// * @note Critical information fields: If present, this is a critical
/// * information field as defined in 5.2.6. An implementation that does not
/// * recognize the indicated CHOICE when verifying a signed SPDU shall indicate
/// * that the signed SPDU is invalid indicate that the signed SPDU is invalid
/// * in the sense of 4.2.2.3.2, that is, it is invalid in the sense that its
/// * validity cannot be established.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to the EccP256CurvePoint and the Ecc384CurvePoint. Both forms of
/// * point are encoded in compressed form, i.e., such that the choice indicated
/// * within the Ecc*CurvePoint is compressed-y-0 or compressed-y-1.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum PublicVerificationKey {
    EcdsaNistP256(EccP256CurvePoint),
    EcdsaBrainpoolP256r1(EccP256CurvePoint),
    #[rasn(extension_addition)]
    EcdsaBrainpoolP384r1(EccP384CurvePoint),
    #[rasn(extension_addition)]
    EcdsaNistP384(EccP384CurvePoint),
    #[rasn(extension_addition)]
    EcsigSm2(EccP256CurvePoint),
}

///*
/// * @brief This structure specifies a rectangle on the surface of the WGS84 ellipsoid where the
/// * sides are given by lines of constant latitude or longitude.
/// * A point which contains an elevation component is considered to be within the rectangular region
/// * if its horizontal projection onto the reference ellipsoid lies within the region.
/// * A RectangularRegion is invalid if the northWest value is south of the southEast value, or if the
/// * latitude values in the two points are equal, or if the longitude values in the two points are
/// * equal; otherwise it is valid. A certificate that contains an invalid RectangularRegion is invalid.
/// *
/// * @param northWest: is the north-west corner of the rectangle.
/// *
/// * @param southEast is the south-east corner of the rectangle.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RectangularRegion {
    pub north_west: TwoDLocation,
    pub south_east: TwoDLocation,
}

impl RectangularRegion {
    pub fn new(north_west: TwoDLocation, south_east: TwoDLocation) -> Self {
        Self {
            north_west,
            south_east,
        }
    }
}

///*
/// * @brief The meanings of the fields in this structure are to be interpreted
/// * in the context of a country within which the region is located, referred
/// * to as the "enclosing country". If this structure is used in a
/// * CountryAndSubregions structure, the enclosing country is the one indicated
/// * by the country field in the CountryAndSubregions structure. If other uses
/// * are defined for this structure in future, it is expected that that
/// * definition will include a specification of how the enclosing country can
/// * be determined.
/// * If the enclosing country is the United States of America:
/// * - The region field identifies the state or statistically equivalent
/// * entity using the integer version of the 2010 FIPS codes as provided by the
/// * U.S. Census Bureau (see normative references in Clause 0).   
/// * - The values in the subregions field identify the county or county
/// * equivalent entity using the integer version of the 2010 FIPS codes as
/// * provided by the U.S. Census Bureau.
/// * If the enclosing country is a different country from the USA, the meaning
/// * of regionAndSubregions is not defined in this version of this standard.
/// * A conformant implementation that implements this type shall recognize (in
/// * the sense of "be able to determine whether a two-dimensional location lies
/// * inside or outside the borders identified by"), for at least one enclosing
/// * country, at least one value for a region within that country and at least
/// * one subregion for the indicated region. In this version of this standard,
/// * the only means to satisfy this is for a conformant implementation to
/// * recognize, for the USA, at least one of the FIPS state codes for US
/// * states, and at least one of the county codes in at least one of the
/// * recognized states. The Protocol Implementation Conformance Statement
/// * (PICS) provided in Annex A allows an implementation to state which
/// * UnCountryId values it recognizes and which region values are recognized
/// * within that country.
/// * If a verifying implementation is required to check that an relevant
/// * geographic information in a signed SPDU is consistent with a certificate
/// * containing one or more instances of this type, then the SDS is permitted
/// * to indicate that the signed SPDU is valid even if some values within
/// * subregions are unrecognized in the sense defined above, so long as the
/// * recognized instances of this type completely contain the relevant
/// * geographic information. Informally, if the recognized values in the
/// * certificate allow the SDS to determine that the SPDU is valid, then it
/// * can make that determination even if there are also unrecognized values
/// * in the certificate. This field is therefore not not a "critical
/// * information field" as defined in 5.2.6, because unrecognized values are
/// * permitted so long as the validity of the SPDU can be established with the
/// * recognized values. However, as discussed in 5.2.6, the presence of an
/// * unrecognized value in a certificate can make it impossible to determine
/// * whether the certificate is valid and so whether the SPDU is valid.
/// * In this structure:
/// *
/// * @param region: identifies a region within a country.
/// *
/// * @param subregions: identifies one or more subregions within region. A
/// * conformant implementation that supports RegionAndSubregions shall support
/// * a subregions field containing at least eight entries.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RegionAndSubregions {
    pub region: Uint8,
    pub subregions: SequenceOfUint16,
}

impl RegionAndSubregions {
    pub fn new(region: Uint8, subregions: SequenceOfUint16) -> Self {
        Self { region, subregions }
    }
}

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfHashedId3(pub SequenceOf<HashedId3>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfIdentifiedRegion(pub SequenceOf<IdentifiedRegion>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfLinkageSeed(pub SequenceOf<LinkageSeed>);

/// Anonymous SEQUENCE OF member

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct AnonymousSequenceOfOctetString(pub OctetString);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfOctetString(pub SequenceOf<AnonymousSequenceOfOctetString>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfPsid(pub SequenceOf<Psid>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfPsidSsp(pub SequenceOf<PsidSsp>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfPsidSspRange(pub SequenceOf<PsidSspRange>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfRectangularRegion(pub SequenceOf<RectangularRegion>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfRegionAndSubregions(pub SequenceOf<RegionAndSubregions>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfUint16(pub SequenceOf<Uint16>);

///*
/// * @brief This type is used for clarity of definitions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfUint8(pub SequenceOf<Uint8>);

///*
/// * @brief This structure represents the Service Specific Permissions (SSP)
/// * relevant to a given entry in a PsidSsp. The meaning of the SSP is specific
/// * to the associated Psid. SSPs may be PSID-specific octet strings or
/// * bitmap-based. See Annex C for further discussion of how application
/// * specifiers may choose which SSP form to use.
/// *
/// * @note Consistency with issuing certificate: If a certificate has an
/// * appPermissions entry A for which the ssp field is opaque, A is consistent
/// * with the issuing certificate if the issuing certificate contains one of
/// * the following:
/// *   - (OPTION 1) A SubjectPermissions field indicating the choice all and
/// * no PsidSspRange field containing the psid field in A;
/// *   - (OPTION 2) A PsidSspRange P for which the following holds:
/// *     - The psid field in P is equal to the psid field in A and one of the
/// * following is true:
/// *       - The sspRange field in P indicates all.
/// *       - The sspRange field in P indicates opaque and one of the entries in
/// * the opaque field in P is an OCTET STRING identical to the opaque field in
/// * A.
/// *
/// * For consistency rules for other types of ServiceSpecificPermissions,
/// * see the following subclauses.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ServiceSpecificPermissions {
    Opaque(OctetString),
    #[rasn(extension_addition)]
    BitmapSsp(BitmapSsp),
}

///***************************************************************************
///                            Crypto Structures                              
///***************************************************************************
///*
/// * @brief This structure represents a signature for a supported public key
/// * algorithm. It may be contained within SignedData or Certificate.
/// *
/// * @note Critical information fields: If present, this is a critical
/// * information field as defined in 5.2.5. An implementation that does not
/// * recognize the indicated CHOICE for this type when verifying a signed SPDU
/// * shall indicate that the signed SPDU is invalid in the sense of 4.2.2.3.2,
/// * that is, it is invalid in the sense that its validity cannot be
/// * established.
/// *
/// * @note Canonicalization: This data structure is subject to canonicalization
/// * for the relevant operations specified in 6.1.2. The canonicalization
/// * applies to instances of this data structure of form EcdsaP256Signature
/// * and EcdsaP384Signature.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum Signature {
    EcdsaNistP256Signature(EcdsaP256Signature),
    EcdsaBrainpoolP256r1Signature(EcdsaP256Signature),
    #[rasn(extension_addition)]
    EcdsaBrainpoolP384r1Signature(EcdsaP384Signature),
    #[rasn(extension_addition)]
    EcdsaNistP384Signature(EcdsaP384Signature),
    #[rasn(extension_addition)]
    Sm2Signature(EcsigP256Signature),
}

///*
/// * @brief This structure identifies the SSPs associated with a PSID for
/// * which the holder may issue or request certificates.
/// *
/// * @note Consistency with issuing certificate: If a certificate has a
/// * PsidSspRange A for which the ssp field is opaque, A is consistent with
/// * the issuing certificate if the issuing certificate contains one of the
/// * following:
/// *   - (OPTION 1) A SubjectPermissions field indicating the choice all and
/// * no PsidSspRange field containing the psid field in A;
/// *   - (OPTION 2) A PsidSspRange P for which the following holds:
/// *     - The psid field in P is equal to the psid field in A and one of the
/// * following is true:
/// *       - The sspRange field in P indicates all.
/// *       - The sspRange field in P indicates opaque, and the sspRange field in
/// * A indicates opaque, and every OCTET STRING within the opaque in A is a
/// * duplicate of an OCTET STRING within the opaque in P.
/// *
/// * If a certificate has a PsidSspRange A for which the ssp field is all,
/// * A is consistent with the issuing certificate if the issuing certificate
/// * contains a PsidSspRange P for which the following holds:
/// *   - (OPTION 1) A SubjectPermissions field indicating the choice all and
/// * no PsidSspRange field containing the psid field in A;
/// *   - (OPTION 2) A PsidSspRange P for which the psid field in P is equal to
/// * the psid field in A and the sspRange field in P indicates all.
/// *
/// * For consistency rules for other types of SspRange, see the following
/// * subclauses.
/// *
/// * @note The choice "all" may also be indicated by omitting the
/// * SspRange in the enclosing PsidSspRange structure. Omitting the SspRange is
/// * preferred to explicitly indicating "all".
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SspRange {
    Opaque(SequenceOfOctetString),
    All(()),
    #[rasn(extension_addition)]
    BitmapSspRange(BitmapSspRange),
}

///***************************************************************************
///                          Certificate Components                           
///***************************************************************************
///*
/// * @brief This field contains the certificate holder's assurance level, which
/// * indicates the security of both the platform and storage of secret keys as
/// * well as the confidence in this assessment.
/// *
/// * This field is encoded as defined in Table 1, where "A" denotes bit
/// * fields specifying an assurance level, "R" reserved bit fields, and "C" bit
/// * fields specifying the confidence.
/// *
/// * Table 1: Bitwise encoding of subject assurance
/// *
/// * | Bit number     |  7  |  6  |  5  |  4  |  3  |  2  |  1  |  0  |
/// * | -------------- | --- | --- | --- | --- | --- | --- | --- | --- |
/// * | Interpretation |  A  |  A  |  A  |  R  |  R  |  R  |  C  |  C  |
/// *
/// * In Table 1, bit number 0 denotes the least significant bit. Bit 7
/// * to bit 5 denote the device's assurance levels, bit 4 to bit 2 are reserved
/// * for future use, and bit 1 and bit 0 denote the confidence.
/// *
/// * The specification of these assurance levels as well as the
/// * encoding of the confidence levels is outside the scope of the present
/// * standard. It can be assumed that a higher assurance value indicates that
/// * the holder is more trusted than the holder of a certificate with lower
/// * assurance value and the same confidence value.
/// *
/// * @note This field was originally specified in ETSI TS 103 097 and
/// * future uses of this field are anticipated to be consistent with future
/// * versions of that standard.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1"))]
pub struct SubjectAssurance(pub OctetString);

///*
/// * @brief This enumerated value indicates supported symmetric algorithms. The
/// * algorithm identifier identifies both the algorithm itself and a specific
/// * mode of operation. The symmetric algorithms supported in this version of
/// * this standard are AES-128 and SM4. The only mode of operation supported is
/// * Counter Mode Encryption With Cipher Block Chaining Message Authentication
/// * Code (CCM). Full details are given in 5.3.8.
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum SymmAlgorithm {
    Aes128Ccm = 0,
    #[rasn(extension_addition)]
    Sm4Ccm = 1,
}

///*
/// * @brief This structure provides the key bytes for use with an identified
/// * symmetric algorithm. The supported symmetric algorithms are AES-128 and
/// * SM4 in CCM mode as specified in 5.3.8.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SymmetricEncryptionKey {
    #[rasn(size("16"))]
    Aes128Ccm(OctetString),
    #[rasn(extension_addition, size("16"))]
    Sm4Ccm(OctetString),
}

///*
/// * @brief This structure contains an estimate of 3D location. The details of
/// * the structure are given in the definitions of the individual fields below.
/// *
/// * @note The units used in this data structure are consistent with the
/// * location data structures used in 	SAE J2735 [B26], though the encoding is
/// * incompatible.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ThreeDLocation {
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub elevation: Elevation,
}

impl ThreeDLocation {
    pub fn new(latitude: Latitude, longitude: Longitude, elevation: Elevation) -> Self {
        Self {
            latitude,
            longitude,
            elevation,
        }
    }
}

///***************************************************************************
///                             Time Structures                               
///***************************************************************************
///*
/// * @brief This type gives the number of (TAI) seconds since 00:00:00 UTC, 1
/// * January, 2004.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Time32(pub Uint32);

///*
/// * @brief This data structure is a 64-bit integer giving an estimate of the
/// * number of (TAI) microseconds since 00:00:00 UTC, 1 January, 2004.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Time64(pub Uint64);

///*
/// * @brief This structure is used to define validity regions for use in
/// * certificates. The latitude and longitude fields contain the latitude and
/// * longitude as defined above.
/// *
/// * @note This data structure is consistent with the location encoding
/// * used in SAE J2735, except that values 900 000 001 for latitude (used to
/// * indicate that the latitude was not available) and 1 800 000 001 for
/// * longitude (used to indicate that the longitude was not available) are not
/// * valid.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TwoDLocation {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

impl TwoDLocation {
    pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

///*
/// * @brief This atomic type is used in the definition of other data structures.
/// * It is for non-negative integers up to 65,535, i.e., (hex)ff ff.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct Uint16(pub u16);

///***************************************************************************
///                               Integer Types                               
///***************************************************************************
///*
/// * @brief This atomic type is used in the definition of other data structures.
/// * It is for non-negative integers up to 7, i.e., (hex)07.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct Uint3(pub u8);

///*
/// * @brief This atomic type is used in the definition of other data structures.
/// * It is for non-negative integers up to 4,294,967,295, i.e.,
/// * (hex)ff ff ff ff.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4294967295"))]
pub struct Uint32(pub u32);

///*
/// * @brief This atomic type is used in the definition of other data structures.
/// * It is for non-negative integers up to 18,446,744,073,709,551,615, i.e.,
/// * (hex)ff ff ff ff ff ff ff ff.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=18446744073709551615"))]
pub struct Uint64(pub u64);

///*
/// * @brief This atomic type is used in the definition of other data structures.
/// * It is for non-negative integers up to 255, i.e., (hex)ff.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Uint8(pub u8);

///*
/// * @brief This type contains the integer representation of the country or
/// * area identifier as defined by the United Nations Statistics Division in
/// * October 2013 (see normative references in Clause 0).
/// * A conformant implementation that implements IdentifiedRegion shall
/// * recognize (in the sense of be able to determine whether a two dimensional
/// * location lies inside or outside the borders identified by) at least one
/// * value of UnCountryId. The Protocol Implementation Conformance Statement
/// * (PICS) provided in Annex A allows an implementation to state which
/// * UnCountryId values it recognizes.
/// * Since 2013 and before the publication of this version of this standard,
/// * three changes have been made to the country code list, to define the
/// * region "sub-Saharan Africa" and remove the "developed regions", and
/// * "developing regions". A conformant implementation may recognize these
/// * region identifiers in the sense defined in the previous paragraph.
/// * If a verifying implementation is required to check that relevant
/// * geographic information in a signed SPDU is consistent with a certificate
/// * containing one or more instances of this type, then the SDS is permitted
/// * to indicate that the signed SPDU is valid even if some instances of this
/// * type are unrecognized in the sense defined above, so long as the
/// * recognized instances of this type completely contain the relevant
/// * geographic information. Informally, if the recognized values in the
/// * certificate allow the SDS to determine that the SPDU is valid, then it
/// * can make that determination even if there are also unrecognized values in
/// * the certificate. This field is therefore not a "critical information
/// * field" as defined in 5.2.6, because unrecognized values are permitted so
/// * long as the validity of the SPDU can be established with the recognized
/// * values. However, as discussed in 5.2.6, the presence of an unrecognized
/// * value in a certificate can make it impossible to determine whether the
/// * certificate and the SPDU are valid.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct UnCountryId(pub Uint16);

///*
/// * @brief The value 900,000,001 indicates that the latitude was not
/// * available to the sender.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("900000001"))]
pub struct UnknownLatitude(pub NinetyDegreeInt);

///*
/// * @brief The value 1,800,000,001 indicates that the longitude was not
/// * available to the sender.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("1800000001"))]
pub struct UnknownLongitude(pub OneEightyDegreeInt);

///*
/// * @brief This type gives the validity period of a certificate. The start of
/// * the validity period is given by start and the end is given by
/// * start + duration.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ValidityPeriod {
    pub start: Time32,
    pub duration: Duration,
}

impl ValidityPeriod {
    pub fn new(start: Time32, duration: Duration) -> Self {
        Self { start, duration }
    }
}
