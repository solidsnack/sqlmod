use self :: RuleResult :: { Matched , Failed } ; use lines :: * ; use query :: * ; fn escape_default ( s : & str ) -> String {
s . chars (  ) . flat_map ( | c | c . escape_default (  ) ) . collect (  ) }
fn char_range_at ( s : & str , pos : usize ) -> ( char , usize ) {
let c = & s [ pos .. ] . chars (  ) . next (  ) . unwrap (  ) ; let next_pos =
pos + c . len_utf8 (  ) ; ( * c , next_pos ) } # [ derive ( Clone ) ] enum
RuleResult < T > { Matched ( usize , T ) , Failed , } # [
derive ( PartialEq , Eq , Debug , Clone ) ] pub struct ParseError {
pub line : usize , pub column : usize , pub offset : usize , pub expected : ::
std :: collections :: HashSet < & 'static str > , } pub type ParseResult < T >
= Result < T , ParseError > ; impl :: std :: fmt :: Display for ParseError {
fn fmt ( & self , fmt : & mut :: std :: fmt :: Formatter ) -> :: std :: result
:: Result < (  ) , :: std :: fmt :: Error > {
try ! (
write ! ( fmt , "error at {}:{}: expected " , self . line , self . column ) )
; if self . expected . len (  ) == 0 { try ! ( write ! ( fmt , "EOF" ) ) ; }
else if self . expected . len (  ) == 1 {
try ! (
write ! (
fmt , "`{}`" , escape_default (
self . expected . iter (  ) . next (  ) . unwrap (  ) ) ) ) ; } else {
let mut iter = self . expected . iter (  ) ; try ! (
write ! (
fmt , "one of `{}`" , escape_default ( iter . next (  ) . unwrap (  ) ) ) ) ;
for elem in iter {
try ! ( write ! ( fmt , ", `{}`" , escape_default ( elem ) ) ) ; } } Ok ( (  )
) } } impl :: std :: error :: Error for ParseError {
fn description ( & self ) -> & str { "parse error" } } fn slice_eq (
input : & str , state : & mut ParseState , pos : usize , m : & 'static str )
-> RuleResult < (  ) > {
# ! [ inline ] # ! [ allow ( dead_code ) ] let l = m . len (  ) ; if input .
len (  ) >= pos + l && & input . as_bytes (  ) [ pos .. pos + l ] == m .
as_bytes (  ) { Matched ( pos + l , (  ) ) } else {
state . mark_failure ( pos , m ) } } fn slice_eq_case_insensitive (
input : & str , state : & mut ParseState , pos : usize , m : & 'static str )
-> RuleResult < (  ) > {
# ! [ inline ] # ! [ allow ( dead_code ) ] let mut used = 0usize ; let mut
input_iter = input [ pos .. ] . chars (  ) . flat_map (
| x | x . to_uppercase (  ) ) ; for m_char_upper in m . chars (  ) . flat_map
( | x | x . to_uppercase (  ) ) {
used += m_char_upper . len_utf8 (  ) ; let input_char_result = input_iter .
next (  ) ; if input_char_result . is_none (  ) || input_char_result . unwrap
(  ) != m_char_upper { return state . mark_failure ( pos , m ) ; } } Matched (
pos + used , (  ) ) } fn any_char (
input : & str , state : & mut ParseState , pos : usize ) -> RuleResult < (  )
> {
# ! [ inline ] # ! [ allow ( dead_code ) ] if input . len (  ) > pos {
let ( _ , next ) = char_range_at ( input , pos ) ; Matched ( next , (  ) ) }
else { state . mark_failure ( pos , "<character>" ) } } fn pos_to_line (
input : & str , pos : usize ) -> ( usize , usize ) {
let mut remaining = pos ; let mut lineno : usize = 1 ; for line in input .
lines (  ) {
let line_length = line . len (  ) + 1 ; if remaining < line_length {
return ( lineno , remaining + 1 ) ; } remaining -= line_length ; lineno += 1 ;
} return ( lineno , remaining + 1 ) ; } impl < 'input > ParseState < 'input >
{
fn mark_failure ( & mut self , pos : usize , expected : & 'static str ) ->
RuleResult < (  ) > {
if pos > self . max_err_pos {
self . max_err_pos = pos ; self . expected . clear (  ) ; } if pos == self .
max_err_pos { self . expected . insert ( expected ) ; } Failed } } struct ParseState < 'input > { max_err_pos : usize , expected : :: std :: collections :: HashSet < & 'static str > , _phantom : :: std :: marker :: PhantomData < & 'input ( ) > , } impl < 'input > ParseState < 'input > { fn new ( ) -> ParseState < 'input > { ParseState { max_err_pos : 0 , expected : :: std :: collections :: HashSet :: new ( ) , _phantom : :: std :: marker :: PhantomData , } } } 

 fn parse_lines < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < Vec<Line<'input>> > { # ! [ allow ( non_snake_case , unused ) ] { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let __pos = if repeat_value . len ( ) > 0 { let sep_res = parse_br ( __input , __state , __pos ) ; match sep_res { Matched ( newpos , _ ) => { newpos } , Failed => break , } } else { __pos } ; let step_res = parse_inline ( __input , __state , __pos ) ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } Matched ( __repeat_pos , repeat_value ) } } 

 fn parse_inline < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < Line<'input> > { # ! [ allow ( non_snake_case , unused ) ] { let choice_res = parse_declaration ( __input , __state , __pos ) ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => { let choice_res = parse_broken_declaration ( __input , __state , __pos ) ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => { let choice_res = parse_comment ( __input , __state , __pos ) ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => parse_something ( __input , __state , __pos ) } } } } } } } 

 fn parse_declaration < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < Line<'input> > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = Matched ( __pos , __pos ) ; match seq_res { Matched ( __pos , start ) => { { let seq_res = { let __assert_res = { let str_start = __pos ; match parse_decl ( __input , __state , __pos ) { Matched ( newpos , _ ) => { Matched ( newpos , & __input [ str_start .. newpos ] ) } , Failed => Failed , } } ; match __assert_res { Matched ( _ , __value ) => Matched ( __pos , __value ) , Failed => Failed , } } ; match seq_res { Matched ( __pos , s ) => { { let seq_res = parse_decl ( __input , __state , __pos ) ; match seq_res { Matched ( __pos , signature ) => { { let seq_res = Matched ( __pos , __pos ) ; match seq_res { Matched ( __pos , end ) => { Matched ( __pos , {  Declaration(Section(start, end, s), signature)  } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn parse_decl < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < Signature > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = slice_eq ( __input , __state , __pos , "--@" ) ; match seq_res { Matched ( __pos , _ ) => { { let seq_res = { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t]" ) , } } else { __state . mark_failure ( __pos , "[ \t]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } if repeat_value . len ( ) >= 1usize { Matched ( __repeat_pos , ( ) ) } else { Failed } } ; match seq_res { Matched ( __pos , _ ) => { { let seq_res = parse_signature ( __input , __state , __pos ) ; match seq_res { Matched ( __pos , signature ) => { { let seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t]" ) , } } else { __state . mark_failure ( __pos , "[ \t]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match seq_res { Matched ( __pos , _ ) => { Matched ( __pos , {  signature  } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn parse_broken_declaration < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < Line<'input> > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = Matched ( __pos , __pos ) ; match seq_res { Matched ( __pos , start ) => { { let seq_res = { let str_start = __pos ; match { let seq_res = slice_eq ( __input , __state , __pos , "--@" ) ; match seq_res { Matched ( __pos , _ ) => { { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { '\r' | '\n' => __state . mark_failure ( __pos , "[^\r\n]" ) , _ => Matched ( __next , ( ) ) , } } else { __state . mark_failure ( __pos , "[^\r\n]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } } Failed => Failed , } } { Matched ( newpos , _ ) => { Matched ( newpos , & __input [ str_start .. newpos ] ) } , Failed => Failed , } } ; match seq_res { Matched ( __pos , s ) => { { let seq_res = Matched ( __pos , __pos ) ; match seq_res { Matched ( __pos , end ) => { Matched ( __pos , {  BrokenDeclaration(Section(start, end, s))  } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn parse_comment < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < Line<'input> > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = Matched ( __pos , __pos ) ; match seq_res { Matched ( __pos , start ) => { { let seq_res = { let str_start = __pos ; match { let seq_res = slice_eq ( __input , __state , __pos , "--" ) ; match seq_res { Matched ( __pos , _ ) => { { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { '\r' | '\n' => __state . mark_failure ( __pos , "[^\r\n]" ) , _ => Matched ( __next , ( ) ) , } } else { __state . mark_failure ( __pos , "[^\r\n]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } } Failed => Failed , } } { Matched ( newpos , _ ) => { Matched ( newpos , & __input [ str_start .. newpos ] ) } , Failed => Failed , } } ; match seq_res { Matched ( __pos , s ) => { { let seq_res = Matched ( __pos , __pos ) ; match seq_res { Matched ( __pos , end ) => { Matched ( __pos , {  Comment(Section(start, end, s))  } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn parse_something < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < Line<'input> > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = Matched ( __pos , __pos ) ; match seq_res { Matched ( __pos , start ) => { { let seq_res = { let str_start = __pos ; match { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { '\r' | '\n' => __state . mark_failure ( __pos , "[^\r\n]" ) , _ => Matched ( __next , ( ) ) , } } else { __state . mark_failure ( __pos , "[^\r\n]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } { Matched ( newpos , _ ) => { Matched ( newpos , & __input [ str_start .. newpos ] ) } , Failed => Failed , } } ; match seq_res { Matched ( __pos , s ) => { { let seq_res = Matched ( __pos , __pos ) ; match seq_res { Matched ( __pos , end ) => { Matched ( __pos , {  Text(Section(start, end, s))  } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn parse_signature < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < Signature > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = parse_name ( __input , __state , __pos ) ; match seq_res { Matched ( __pos , name ) => { { let seq_res = match parse_parenthesized ( __input , __state , __pos ) { Matched ( newpos , value ) => { Matched ( newpos , Some ( value ) ) } , Failed => { Matched ( __pos , None ) } , } ; match seq_res { Matched ( __pos , parenthesized ) => { { let seq_res = match { let seq_res = { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t]" ) , } } else { __state . mark_failure ( __pos , "[ \t]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } if repeat_value . len ( ) >= 1usize { Matched ( __repeat_pos , ( ) ) } else { Failed } } ; match seq_res { Matched ( __pos , _ ) => { { let seq_res = { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let __pos = if repeat_value . len ( ) > 0 { let sep_res = { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t]" ) , } } else { __state . mark_failure ( __pos , "[ \t]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } if repeat_value . len ( ) >= 1usize { Matched ( __repeat_pos , ( ) ) } else { Failed } } ; match sep_res { Matched ( newpos , _ ) => { newpos } , Failed => break , } } else { __pos } ; let step_res = parse_annotation ( __input , __state , __pos ) ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } if repeat_value . len ( ) >= 1usize { Matched ( __repeat_pos , repeat_value ) } else { Failed } } ; match seq_res { Matched ( __pos , inner ) => { Matched ( __pos , {  inner  } ) } Failed => Failed , } } } Failed => Failed , } } { Matched ( newpos , value ) => { Matched ( newpos , Some ( value ) ) } , Failed => { Matched ( __pos , None ) } , } ; match seq_res { Matched ( __pos , annotations ) => { Matched ( __pos , {  let annotations = annotations.unwrap_or(vec![]).into_iter();
        let parenthesized = parenthesized.into_iter();
        let combined = parenthesized.chain(annotations).map(|s| s.into());
        Signature { name: name.into(), attributes: combined.collect() }  } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn parse_name < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < &'input str > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = { let str_start = __pos ; match { let seq_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { 'a' ... 'z' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[a-z]" ) , } } else { __state . mark_failure ( __pos , "[a-z]" ) } ; match seq_res { Matched ( __pos , _ ) => { { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { 'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '_' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[a-zA-Z0-9_]" ) , } } else { __state . mark_failure ( __pos , "[a-zA-Z0-9_]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } } Failed => Failed , } } { Matched ( newpos , _ ) => { Matched ( newpos , & __input [ str_start .. newpos ] ) } , Failed => Failed , } } ; match seq_res { Matched ( __pos , s ) => { Matched ( __pos , {  s  } ) } Failed => Failed , } } } 

 fn parse_annotation < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < &'input str > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = { let str_start = __pos ; match { let choice_res = parse_plain_word ( __input , __state , __pos ) ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => { let choice_res = { let seq_res = parse_plain_word ( __input , __state , __pos ) ; match seq_res { Matched ( __pos , _ ) => { parse_balanced_parens ( __input , __state , __pos ) } Failed => Failed , } } ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => { let choice_res = parse_balanced_parens ( __input , __state , __pos ) ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => parse_quoted ( __input , __state , __pos ) } } } } } } { Matched ( newpos , _ ) => { Matched ( newpos , & __input [ str_start .. newpos ] ) } , Failed => Failed , } } ; match seq_res { Matched ( __pos , s ) => { Matched ( __pos , {  s  } ) } Failed => Failed , } } } 

 fn parse_parenthesized < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < &'input str > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = { let str_start = __pos ; match parse_balanced_parens ( __input , __state , __pos ) { Matched ( newpos , _ ) => { Matched ( newpos , & __input [ str_start .. newpos ] ) } , Failed => Failed , } } ; match seq_res { Matched ( __pos , s ) => { Matched ( __pos , {  s  } ) } Failed => Failed , } } } 

 fn parse_plain_word < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < () > { # ! [ allow ( non_snake_case , unused ) ] { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '"' | '(' | ')' | '\r' | '\n' => __state . mark_failure ( __pos , "[^ \t\"()\r\n]" ) , _ => Matched ( __next , ( ) ) , } } else { __state . mark_failure ( __pos , "[^ \t\"()\r\n]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } if repeat_value . len ( ) >= 1usize { Matched ( __repeat_pos , ( ) ) } else { Failed } } } 

 fn parse_balanced_parens < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < () > { # ! [ allow ( non_snake_case , unused ) ] { let choice_res = slice_eq ( __input , __state , __pos , "()" ) ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => { let choice_res = { let seq_res = slice_eq ( __input , __state , __pos , "(" ) ; match seq_res { Matched ( __pos , _ ) => { { let seq_res = { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { '(' | ')' => __state . mark_failure ( __pos , "[^()]" ) , _ => Matched ( __next , ( ) ) , } } else { __state . mark_failure ( __pos , "[^()]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } if repeat_value . len ( ) >= 1usize { Matched ( __repeat_pos , ( ) ) } else { Failed } } ; match seq_res { Matched ( __pos , _ ) => { slice_eq ( __input , __state , __pos , ")" ) } Failed => Failed , } } } Failed => Failed , } } ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => { let seq_res = slice_eq ( __input , __state , __pos , "(" ) ; match seq_res { Matched ( __pos , _ ) => { { let seq_res = { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let step_res = { let choice_res = { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { '(' | ')' => __state . mark_failure ( __pos , "[^()]" ) , _ => Matched ( __next , ( ) ) , } } else { __state . mark_failure ( __pos , "[^()]" ) } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } if repeat_value . len ( ) >= 1usize { Matched ( __repeat_pos , ( ) ) } else { Failed } } ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => parse_balanced_parens ( __input , __state , __pos ) } } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } if repeat_value . len ( ) >= 1usize { Matched ( __repeat_pos , ( ) ) } else { Failed } } ; match seq_res { Matched ( __pos , _ ) => { slice_eq ( __input , __state , __pos , ")" ) } Failed => Failed , } } } Failed => Failed , } } } } } } } 

 fn parse_quoted < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < () > { # ! [ allow ( non_snake_case , unused ) ] { let seq_res = slice_eq ( __input , __state , __pos , "\"" ) ; match seq_res { Matched ( __pos , _ ) => { { let seq_res = { let mut __repeat_pos = __pos ; let mut repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let step_res = { let choice_res = { let seq_res = slice_eq ( __input , __state , __pos , "\\" ) ; match seq_res { Matched ( __pos , _ ) => { slice_eq ( __input , __state , __pos , "\"" ) } Failed => Failed , } } ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { '"' => __state . mark_failure ( __pos , "[^\"]" ) , _ => Matched ( __next , ( ) ) , } } else { __state . mark_failure ( __pos , "[^\"]" ) } } } ; match step_res { Matched ( newpos , value ) => { __repeat_pos = newpos ; repeat_value . push ( value ) ; } , Failed => { break ; } } } if repeat_value . len ( ) >= 1usize { Matched ( __repeat_pos , ( ) ) } else { Failed } } ; match seq_res { Matched ( __pos , _ ) => { slice_eq ( __input , __state , __pos , "\"" ) } Failed => Failed , } } } Failed => Failed , } } } 

 fn parse_br < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < () > { # ! [ allow ( non_snake_case , unused ) ] { let choice_res = slice_eq ( __input , __state , __pos , "\r\n" ) ; match choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => slice_eq ( __input , __state , __pos , "\n" ) } } } pub fn lines < 'input > ( input : & 'input str ) -> ParseResult < Vec<Line<'input>> > { # ! [ allow ( non_snake_case , unused ) ] let mut state = ParseState :: new ( ) ; match parse_lines ( input , & mut state , 0 ) { Matched ( pos , value ) => { if pos == input . len ( ) { return Ok ( value ) } } _ => { } } let ( line , col ) = pos_to_line ( input , state . max_err_pos ) ; Err ( ParseError { line : line , column : col , offset : state . max_err_pos , expected : state . expected , } ) }
