use crate::*;

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[near_bindgen]
impl Contract{
    pub fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        let owner_id: AccountId = AccountId::new_unchecked(token_id.clone());
        if let Some (points) = self.points.get(&owner_id){
            internal_get_token(token_id, owner_id, points)
        }
        else {
            None
        }
    }

    pub fn nft_token_metadata(&self, token_id: TokenId) -> Option<TokenMetadata> {
        let owner_id: AccountId = AccountId::new_unchecked(token_id);
        if let Some (points) = self.points.get(&owner_id){
            internal_get_token_metadata(points)
        }
        else {
            None
        }
    }

    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> u64 {
        if self.points.get(&account_id).is_some(){
            1
        }
        else {
            0
        }
    }

    pub fn nft_tokens_for_owner(&self, account_id: AccountId) -> Vec<Option<Token>>{
        if let Some (points) = self.points.get(&account_id){
            vec![internal_get_token(account_id.to_string(), account_id, points)]
        }
        else {
            vec![]
        }
    }
}

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

pub fn get_default_meta() -> NFTContractMetadata {
    NFTContractMetadata {
        spec: NFT_METADATA_SPEC.to_string(),
        name: "NEAR JsRank NFT".to_string(),
        symbol: "Hackerrank".to_string(),
        icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
        base_uri: None,
        reference: None,
        reference_hash: None,
    }
}

pub fn internal_get_token_metadata(points: Point) -> Option<TokenMetadata> {
    Some(TokenMetadata {
        title: Some("NEAR JsRank NFT".to_string()),
        description: Some(format!("{} challenges solved", points)),
        media: Some(get_media(points.to_string())),
        media_hash: None,
        copies: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    })
}

pub fn internal_get_token(token_id: TokenId, owner_id: AccountId, points: Point) -> Option<Token> {
    Some(
        Token {
            token_id,
            owner_id,
            metadata: internal_get_token_metadata(points),
            approved_account_ids: None
        })
}

pub fn get_media(points: String) -> String{
    /*
    <svg id="medal" data-name="medal" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 473 473"><title>Medal</title><circle cx="235.971" cy="273.007" r="121.689" fill="#d19000" stroke="#a57005" stroke-miterlimit="10" stroke-width="18"></circle><rect class="light" x="110.246" y="70.854" width="22" height="294.983" transform="translate(111.416 -44.37) rotate(30)" fill="#fafafa" opacity="0.10"></rect><path d="M250.5,13C119.885,13,14,118.885,14,249.5S119.885,486,250.5,486,487,380.115,487,249.5,381.115,13,250.5,13Zm0,424.324A150.789,150.789,0,1,1,401.289,286.536,150.789,150.789,0,0,1,250.5,437.324Z" transform="translate(-14 -13)" fill="#24c7f9"></path><rect x="196.819" y="116.398" width="78.304" height="12.698" fill="#a57005"></rect><path d="M305,112.468H196a8.465,8.465,0,1,0,0,16.931H305a8.465,8.465,0,1,0,0-16.931Zm-10.92,13.756H207.982c-3.693,0-6.687-2.369-6.687-5.291s2.994-5.291,6.687-5.291h86.094c3.693,0,6.687,2.369,6.687,5.291S297.769,126.224,294.076,126.224Z" transform="translate(-14 -13)" fill="#d19000"></path><path d="M326.235,25.388,297.322,125.557a8.062,8.062,0,0,1-3.246.667H207.551a8.062,8.062,0,0,1-3.246-.667L175.338,25.2C81.571,56.61,14,145.161,14,249.5,14,380.115,119.885,486,250.5,486S487,380.115,487,249.5C487,145.372,419.705,56.964,326.235,25.388Z" transform="translate(-14 -13)" fill="none"></path><path d="M241.773,26.425l.009.034,9.031,31.286,12.809-44.377C259.278,13.13,254.9,13,250.5,13q-6.294,0-12.507.329l3.78,13.1Z" transform="translate(-14 -13)" fill="none"></path><polygon points="224.174 88.535 236.813 44.745 227.783 13.459 236.426 44.626 224.174 88.535" fill="#2c2c33"></polygon><path d="M228.324,126.224h2.723l.039-.136C230.4,126.174,228.6,126.224,228.324,126.224Z" transform="translate(-14 -13)" fill="#2c2c33"></path><path d="M207.551,126.224h5.508l28.714-99.8-3.78-13.1A235.648,235.648,0,0,0,175.338,25.2L204.3,125.557A8.062,8.062,0,0,0,207.551,126.224Z" transform="translate(-14 -13)" fill="#2c2c33"></path><path d="M238.174,101.535l-6.838,24.505a1.217,1.217,0,0,1-.249.047l-.039.136h63.028a8.062,8.062,0,0,0,3.246-.667L326.235,25.388a235.629,235.629,0,0,0-62.612-12.02L250.813,57.745Z" transform="translate(-14 -13)" fill="#2c2c33"></path><path d="M231.336,126.04l6.838-24.505,12.252-43.909-8.644-31.167-.009-.034h0l-28.714,99.8h15.266c.278,0,2.077-.049,2.763-.136A1.217,1.217,0,0,0,231.336,126.04Z" transform="translate(-14 -13)" fill="#1e1e23"></path><path d="M250.5,135.747A150.789,150.789,0,1,0,401.289,286.536,150.789,150.789,0,0,0,250.5,135.747ZM250.141,415.7A129.859,129.859,0,1,1,380,285.837,129.859,129.859,0,0,1,250.141,415.7Z" transform="translate(-14 -13)" fill="#d19000"></path>
    <svg id='medal' data-name='medal' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 473 473'><title>Medal</title><circle cx='235.971' cy='273.007' r='121.689' fill='#d19000' stroke='#a57005' stroke-miterlimit='10' stroke-width='18'></circle><rect class='light' x='110.246' y='70.854' width='22' height='294.983' transform='translate(111.416 -44.37) rotate(30)' fill='#fafafa' opacity='0.10'></rect><path d='M250.5,13C119.885,13,14,118.885,14,249.5S119.885,486,250.5,486,487,380.115,487,249.5,381.115,13,250.5,13Zm0,424.324A150.789,150.789,0,1,1,401.289,286.536,150.789,150.789,0,0,1,250.5,437.324Z' transform='translate(-14 -13)' fill='#24c7f9'></path><rect x='196.819' y='116.398' width='78.304' height='12.698' fill='#a57005'></rect><path d='M305,112.468H196a8.465,8.465,0,1,0,0,16.931H305a8.465,8.465,0,1,0,0-16.931Zm-10.92,13.756H207.982c-3.693,0-6.687-2.369-6.687-5.291s2.994-5.291,6.687-5.291h86.094c3.693,0,6.687,2.369,6.687,5.291S297.769,126.224,294.076,126.224Z' transform='translate(-14 -13)' fill='#d19000'></path><path d='M326.235,25.388,297.322,125.557a8.062,8.062,0,0,1-3.246.667H207.551a8.062,8.062,0,0,1-3.246-.667L175.338,25.2C81.571,56.61,14,145.161,14,249.5,14,380.115,119.885,486,250.5,486S487,380.115,487,249.5C487,145.372,419.705,56.964,326.235,25.388Z' transform='translate(-14 -13)' fill='none'></path><path d='M241.773,26.425l.009.034,9.031,31.286,12.809-44.377C259.278,13.13,254.9,13,250.5,13q-6.294,0-12.507.329l3.78,13.1Z' transform='translate(-14 -13)' fill='none'></path><polygon points='224.174 88.535 236.813 44.745 227.783 13.459 236.426 44.626 224.174 88.535' fill='#2c2c33'></polygon><path d='M228.324,126.224h2.723l.039-.136C230.4,126.174,228.6,126.224,228.324,126.224Z' transform='translate(-14 -13)' fill='#2c2c33'></path><path d='M207.551,126.224h5.508l28.714-99.8-3.78-13.1A235.648,235.648,0,0,0,175.338,25.2L204.3,125.557A8.062,8.062,0,0,0,207.551,126.224Z' transform='translate(-14 -13)' fill='#2c2c33'></path><path d='M238.174,101.535l-6.838,24.505a1.217,1.217,0,0,1-.249.047l-.039.136h63.028a8.062,8.062,0,0,0,3.246-.667L326.235,25.388a235.629,235.629,0,0,0-62.612-12.02L250.813,57.745Z' transform='translate(-14 -13)' fill='#2c2c33'></path><path d='M231.336,126.04l6.838-24.505,12.252-43.909-8.644-31.167-.009-.034h0l-28.714,99.8h15.266c.278,0,2.077-.049,2.763-.136A1.217,1.217,0,0,0,231.336,126.04Z' transform='translate(-14 -13)' fill='#1e1e23'></path><path d='M250.5,135.747A150.789,150.789,0,1,0,401.289,286.536,150.789,150.789,0,0,0,250.5,135.747ZM250.141,415.7A129.859,129.859,0,1,1,380,285.837,129.859,129.859,0,0,1,250.141,415.7Z' transform='translate(-14 -13)' fill='#d19000'></path><text x='50%' y='60%' font-size='10em' dominant-baseline='middle' text-anchor='middle' style='fill:black; font-family:arial,serif; font-weight:bold'>1</text></svg>
    <text x="50%" y="60%" font-size="10em" dominant-baseline="middle" text-anchor="middle" style="fill:black; font-family:arial,serif; font-weight:bold">0</text>
    </svg>
     */
    format!(
        "data:image/svg+xml,%3Csvg id='medal' data-name='medal' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 473 473'%3E%3Ctitle%3EMedal%3C/title%3E%3Ccircle cx='235.971' cy='273.007' r='121.689' fill='%23d19000' stroke='%23a57005' stroke-miterlimit='10' stroke-width='18'%3E%3C/circle%3E%3Crect class='light' x='110.246' y='70.854' width='22' height='294.983' transform='translate(111.416 -44.37) rotate(30)' fill='%23fafafa' opacity='0.10'%3E%3C/rect%3E%3Cpath d='M250.5,13C119.885,13,14,118.885,14,249.5S119.885,486,250.5,486,487,380.115,487,249.5,381.115,13,250.5,13Zm0,424.324A150.789,150.789,0,1,1,401.289,286.536,150.789,150.789,0,0,1,250.5,437.324Z' transform='translate(-14 -13)' fill='%2324c7f9'%3E%3C/path%3E%3Crect x='196.819' y='116.398' width='78.304' height='12.698' fill='%23a57005'%3E%3C/rect%3E%3Cpath d='M305,112.468H196a8.465,8.465,0,1,0,0,16.931H305a8.465,8.465,0,1,0,0-16.931Zm-10.92,13.756H207.982c-3.693,0-6.687-2.369-6.687-5.291s2.994-5.291,6.687-5.291h86.094c3.693,0,6.687,2.369,6.687,5.291S297.769,126.224,294.076,126.224Z' transform='translate(-14 -13)' fill='%23d19000'%3E%3C/path%3E%3Cpath d='M326.235,25.388,297.322,125.557a8.062,8.062,0,0,1-3.246.667H207.551a8.062,8.062,0,0,1-3.246-.667L175.338,25.2C81.571,56.61,14,145.161,14,249.5,14,380.115,119.885,486,250.5,486S487,380.115,487,249.5C487,145.372,419.705,56.964,326.235,25.388Z' transform='translate(-14 -13)' fill='none'%3E%3C/path%3E%3Cpath d='M241.773,26.425l.009.034,9.031,31.286,12.809-44.377C259.278,13.13,254.9,13,250.5,13q-6.294,0-12.507.329l3.78,13.1Z' transform='translate(-14 -13)' fill='none'%3E%3C/path%3E%3Cpolygon points='224.174 88.535 236.813 44.745 227.783 13.459 236.426 44.626 224.174 88.535' fill='%232c2c33'%3E%3C/polygon%3E%3Cpath d='M228.324,126.224h2.723l.039-.136C230.4,126.174,228.6,126.224,228.324,126.224Z' transform='translate(-14 -13)' fill='%232c2c33'%3E%3C/path%3E%3Cpath d='M207.551,126.224h5.508l28.714-99.8-3.78-13.1A235.648,235.648,0,0,0,175.338,25.2L204.3,125.557A8.062,8.062,0,0,0,207.551,126.224Z' transform='translate(-14 -13)' fill='%232c2c33'%3E%3C/path%3E%3Cpath d='M238.174,101.535l-6.838,24.505a1.217,1.217,0,0,1-.249.047l-.039.136h63.028a8.062,8.062,0,0,0,3.246-.667L326.235,25.388a235.629,235.629,0,0,0-62.612-12.02L250.813,57.745Z' transform='translate(-14 -13)' fill='%232c2c33'%3E%3C/path%3E%3Cpath d='M231.336,126.04l6.838-24.505,12.252-43.909-8.644-31.167-.009-.034h0l-28.714,99.8h15.266c.278,0,2.077-.049,2.763-.136A1.217,1.217,0,0,0,231.336,126.04Z' transform='translate(-14 -13)' fill='%231e1e23'%3E%3C/path%3E%3Cpath d='M250.5,135.747A150.789,150.789,0,1,0,401.289,286.536,150.789,150.789,0,0,0,250.5,135.747ZM250.141,415.7A129.859,129.859,0,1,1,380,285.837,129.859,129.859,0,0,1,250.141,415.7Z' transform='translate(-14 -13)' fill='%23d19000'%3E%3C/path%3E%3Ctext x='50%25' y='60%25' font-size='10em' dominant-baseline='middle' text-anchor='middle' style='fill:black; font-family:arial,serif; font-weight:bold'%3E{}%3C/text%3E%3C/svg%3E",
        points
    )
}