use literals::*;
use arguments::*;
use common::*;
use Parse;
use types::*;
use attributes::*;

/// CallbackOrInterfaceOrMixin ::
///     callback CallbackRestOrInterface
///     interface InterfaceOrMixin
#[derive(Debug, PartialEq)]
pub enum CallbackOrInterfaceOrMixin {
    Callback(CallbackRestOrInterfacePart),
    Interface(InterfaceOrMixinPart),
}

#[derive(Debug, PartialEq)]
pub struct CallbackRestOrInterfacePart {
    pub callback: term!(callback),
    pub rest: CallbackRestOrInterface,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceOrMixinPart {
    pub interface: term!(interface),
    pub rest: InterfaceOrMixin,
}

/// CallbackRestOrInterface ::
///     CallbackRest
///     interface InterfaceRest
#[derive(Debug, PartialEq)]
pub enum CallbackRestOrInterface {
    CallbackRest(CallbackRest),
    Interface(InterfaceRestPart),
}

#[derive(Debug, PartialEq)]
pub struct InterfaceRestPart {
    pub interface: term!(interface),
    pub rest: InterfaceRest,
}

/// CallbackRest ::
///     **identifier** = ReturnType ( ArgumentList ) ;
#[derive(Debug, PartialEq)]
pub struct CallbackRest {
    pub identifier: Identifier,
    pub assign: term!(=),
    pub return_type: ReturnType,
    pub braced: Braced<ArgumentList>,
}

impl Parse for CallbackRest {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        return_type: weedle!(ReturnType) >>
        braced: weedle!(Braced<ArgumentList>) >>
        (CallbackRest { identifier, assign, return_type, braced })
    ));
}

/// InterfaceRest ::
///     **identifier** Inheritance { InterfaceMembers } ;
#[derive(Debug, PartialEq)]
pub struct InterfaceRest {
    pub identifier: Identifier,
    pub inheritance: Option<Inheritance>,
    pub parenthesized: Parenthesized<InterfaceMembers>,
    pub semi_colon: term!(;),
}

/// Inheritance ::
///     : **identifier**
///     ε
///
/// Since it is optional, Option<Inheritance> be used
#[derive(Debug, PartialEq)]
pub struct Inheritance {
    pub colon: term!(:),
    pub identifier: Identifier,
}

/// InterfaceMembers ::
///     ExtendedAttributeList InterfaceMember InterfaceMembers
///     ε
#[derive(Debug, PartialEq)]
pub struct InterfaceMembers {
    pub members: Vec<InterfaceMembersItem>
}

#[derive(Debug, PartialEq)]
pub struct InterfaceMembersItem {
    pub attributes: ExtendedAttributeList,
    pub member: InterfaceMember,
}

/// InterfaceMember ::
///     Const
///     Operation
///     Stringifier
///     StaticMember
///     Iterable
///     ReadOnlyMember
///     ReadWriteAttribute
///     ReadWriteMaplike
///     ReadWriteSetlike
#[derive(Debug, PartialEq)]
pub enum InterfaceMember {
    Const(ConstItem),
    Operation(Operation),
    Stringifier(StringifierItem),
    StaticMember(StaticMember),
    Iterable(IterableItem),
    ReadOnlyMember(ReadOnlyMember),
    ReadWriteAttribute(ReadWriteAttribute),
    ReadWriteMaplike(ReadWriteMaplike),
    ReadWriteSetlike(ReadWriteSetlike),
}

/// Const ::
///     const ConstType identifier = ConstValue ;
#[derive(Debug, PartialEq)]
pub struct ConstItem {
    pub const_: term!(const),
    pub const_type: ConstType,
    pub identifier: Identifier,
    pub assign: term!(=),
    pub const_value: ConstValue,
    pub semi_colon: term!(;)
}

/// Operation ::
///     RegularOperation
///     SpecialOperation
#[derive(Debug, PartialEq)]
pub enum Operation {
    Regular(RegularOperation),
    Special(SpecialOperation)
}

/// RegularOperation ::
///    ReturnType OperationRest
#[derive(Debug, PartialEq)]
pub struct RegularOperation {
    pub return_type: ReturnType,
    pub rest: OperationRest
}

/// OperationRest ::
///     OptionalIdentifier ( ArgumentList ) ;
#[derive(Debug, PartialEq)]
pub struct OperationRest {
    pub identifier: Option<Identifier>,
    pub braced: Braced<ArgumentList>
}

/// SpecialOperation ::
///     Special Specials RegularOperation
#[derive(Debug, PartialEq)]
pub struct SpecialOperation {
    pub specials: Vec<Special>,
    pub regular_operation: RegularOperation
}

/// Special ::
///     getter
///     setter
///     deleter
#[derive(Debug, PartialEq)]
pub enum Special {
    Getter(term!(getter)),
    Setter(term!(setter)),
    Deleter(term!(deleter))
}

/// Stringifier ::
///     stringifier StringifierRest
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Stringifier)
#[derive(Debug, PartialEq)]
pub struct StringifierItem {
    pub stringifier: term!(stringifier),
    pub rest: StringifierRest
}

/// StringifierRest ::
///     ReadOnly AttributeRest
///     RegularOperation
///     ;
///
/// ReadOnly ::
///     readonly
///     ε
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-StringifierRest)
#[derive(Debug, PartialEq)]
pub enum StringifierRest {
    ReadOnly(ReadOnlyAttributeRest),
    RegularOperation(RegularOperation),
    SemiColon(term!(;))
}

#[derive(Debug, PartialEq)]
pub struct ReadOnlyAttributeRest {
    pub readonly: Option<term!(readonly)>,
    pub rest: AttributeRest
}

/// StaticMember ::
///     static StaticMemberRest
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-StaticMember)
#[derive(Debug, PartialEq)]
pub struct StaticMember {
    pub static_: term!(static),
    pub rest: StaticMemberRest
}

/// StaticMemberRest ::
///     ReadOnly AttributeRest
///     RegularOperation
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-StaticMemberRest)
#[derive(Debug, PartialEq)]
pub enum StaticMemberRest {
    ReadOnly(ReadOnlyAttributeRest),
    RegularOperation(RegularOperation)
}

/// Iterable ::
///     iterable < TypeWithExtendedAttributes OptionalType > ;
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Iterable)
#[derive(Debug, PartialEq)]
pub struct IterableItem {
    pub iterable: term!(iterable),
    pub generics: Generics<IterableGenericsType>
}

#[derive(Debug, PartialEq)]
pub struct IterableGenericsType {
    pub type_: TypeWithExtendedAttributes,
    pub rest: Option<IterableGenericsTypeRest>
}

/// OptionalType ::
///     , TypeWithExtendedAttributes
///     ε
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-OptionalType)
#[derive(Debug, PartialEq)]
pub struct IterableGenericsTypeRest {
    pub comma: term!(,),
    pub type_: TypeWithExtendedAttributes
}

/// ReadOnlyMember ::
///     readonly ReadOnlyMemberRest
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadOnlyMember)
#[derive(Debug, PartialEq)]
pub struct ReadOnlyMember {
    pub readonly: term!(readonly),
    pub rest: ReadOnlyMemberRest
}

/// ReadOnlyMemberRest ::
///     AttributeRest
///     ReadWriteMaplike
///     ReadWriteSetlike
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadOnlyMemberRest)
#[derive(Debug, PartialEq)]
pub enum ReadOnlyMemberRest {
    AttributeRest(AttributeRest),
    ReadWriteMaplike(ReadWriteMaplike),
    ReadWriteSetlike(ReadWriteSetlike)
}

/// ReadWriteMaplike ::
///     MaplikeRest
///
/// MaplikeRest ::
///     maplike < TypeWithExtendedAttributes , TypeWithExtendedAttributes > ;
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadWriteMaplike)
#[derive(Debug, PartialEq)]
pub struct ReadWriteMaplike {
    pub maplike: term!(maplike),
    pub generics: Generics<MaplikeGenericsType>,
    pub semi_colon: term!(;)
}

#[derive(Debug, PartialEq)]
pub struct MaplikeGenericsType {
    pub type_1: TypeWithExtendedAttributes,
    pub comma: term!(,),
    pub type_2: TypeWithExtendedAttributes
}

/// ReadWriteSetlike ::
///     SetlikeRest
///
/// SetlikeRest ::
///     setlike < TypeWithExtendedAttributes > ;
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadWriteSetlike)
#[derive(Debug, PartialEq)]
pub struct ReadWriteSetlike {
    pub setlike: term!(setlike),
    pub generics: Generics<TypeWithExtendedAttributes>,
    pub semi_colon: term!(;)
}

/// ReadWriteAttribute ::
///     inherit ReadOnly AttributeRest
///     AttributeRest
///
/// ReadOnly ::
///     readonly
///     ε
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadWriteAttribute)
#[derive(Debug, PartialEq)]
pub enum ReadWriteAttribute {
    Inherit(InheritAttribute),
    AttributeRest(AttributeRest)
}

#[derive(Debug, PartialEq)]
pub struct InheritAttribute {
    pub inherit: term!(inherit),
    pub readonly: Option<term!(readonly)>,
    pub rest: AttributeRest
}

/// InterfaceOrMixin ::
///     InterfaceRest
///     MixinRest
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-InterfaceOrMixin)
#[derive(Debug, PartialEq)]
pub enum InterfaceOrMixin {
    InterfaceRest(InterfaceRest),
    MixinRest(MixinRest)
}

/// MixinRest ::
///     mixin **identifier** { MixinMembers } ;
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-MixinRest)
#[derive(Debug, PartialEq)]
pub struct MixinRest {
    pub mixin: term!(mixin),
    pub identifier: Identifier,
    pub parenthesized: Parenthesized<MixinMembers>
}

/// MixinMembers ::
///     ExtendedAttributeList MixinMember MixinMembers
///     ε
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-MixinMembers)
#[derive(Debug, PartialEq)]
pub struct MixinMembers {
    members: Vec<MixinMembersItem>
}

#[derive(Debug, PartialEq)]
pub struct MixinMembersItem {
    pub attributes: ExtendedAttributeList,
    pub member: MixinMember
}

/// MixinMember ::
///     Const
///     RegularOperation
///     Stringifier
///     ReadOnly AttributeRest
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-MixinMember)
#[derive(Debug, PartialEq)]
pub enum MixinMember {
    Const(ConstItem),
    RegularOperation(RegularOperation),
    Stringifier(StringifierItem),
    ReadOnly(ReadOnlyAttributeRest)
}