use std::fmt;

use super::super::sprites::ISprite;

impl fmt::Display for FontAwesome5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ISprite for FontAwesome5 {
    fn export(&self) -> (String, Vec<String>) {
        let name = self.to_string();
        let include_sprite =
            format!("!include <tupadr3/font-awesome-5/{}>", name);
        (
            name,
            vec!["!include <tupadr3/common>".into(), include_sprite],
        )
    }
}

#[derive(Clone, Debug)]
pub enum FontAwesome5 {
    _500px,
    _box,
    accessible_icon,
    accusoft,
    acquisitions_incorporated,
    ad,
    address_book,
    address_card,
    adjust,
    adn,
    adversal,
    affiliatetheme,
    air_freshener,
    airbnb,
    algolia,
    align_center,
    align_justify,
    align_left,
    align_right,
    alipay,
    allergies,
    amazon,
    amazon_pay,
    ambulance,
    american_sign_language_interpreting,
    amilia,
    anchor,
    android,
    angellist,
    angle_double_down,
    angle_double_left,
    angle_double_right,
    angle_double_up,
    angle_down,
    angle_left,
    angle_right,
    angle_up,
    angry,
    angrycreative,
    angular,
    ankh,
    app_store,
    app_store_ios,
    apper,
    apple,
    apple_alt,
    apple_pay,
    archive,
    archway,
    arrow_alt_circle_down,
    arrow_alt_circle_left,
    arrow_alt_circle_right,
    arrow_alt_circle_up,
    arrow_circle_down,
    arrow_circle_left,
    arrow_circle_right,
    arrow_circle_up,
    arrow_down,
    arrow_left,
    arrow_right,
    arrow_up,
    arrows_alt,
    arrows_alt_h,
    arrows_alt_v,
    artstation,
    assistive_listening_systems,
    asterisk,
    asymmetrik,
    at,
    atlas,
    atlassian,
    atom,
    audible,
    audio_description,
    autoprefixer,
    avianex,
    aviato,
    award,
    aws,
    baby,
    baby_carriage,
    backspace,
    backward,
    bacon,
    bacteria,
    bacterium,
    bahai,
    balance_scale,
    balance_scale_left,
    balance_scale_right,
    ban,
    band_aid,
    bandcamp,
    barcode,
    bars,
    baseball_ball,
    basketball_ball,
    bath,
    battery_empty,
    battery_full,
    battery_half,
    battery_quarter,
    battery_three_quarters,
    battle_net,
    bed,
    beer,
    behance,
    behance_square,
    bell,
    bell_slash,
    bezier_curve,
    bible,
    bicycle,
    biking,
    bimobject,
    binoculars,
    biohazard,
    birthday_cake,
    bitbucket,
    bitcoin,
    bity,
    black_tie,
    blackberry,
    blender,
    blender_phone,
    blind,
    blog,
    blogger,
    blogger_b,
    bluetooth,
    bluetooth_b,
    bold,
    bolt,
    bomb,
    bone,
    bong,
    book,
    book_dead,
    book_medical,
    book_open,
    book_reader,
    bookmark,
    bootstrap,
    border_all,
    border_none,
    border_style,
    bowling_ball,
    box_open,
    box_tissue,
    boxes,
    braille,
    brain,
    bread_slice,
    briefcase,
    briefcase_medical,
    broadcast_tower,
    broom,
    brush,
    btc,
    buffer,
    bug,
    building,
    bullhorn,
    bullseye,
    burn,
    buromobelexperte,
    bus,
    bus_alt,
    business_time,
    buy_n_large,
    buysellads,
    calculator,
    calendar,
    calendar_alt,
    calendar_check,
    calendar_day,
    calendar_minus,
    calendar_plus,
    calendar_times,
    calendar_week,
    camera,
    camera_retro,
    campground,
    canadian_maple_leaf,
    candy_cane,
    cannabis,
    capsules,
    car,
    car_alt,
    car_battery,
    car_crash,
    car_side,
    caravan,
    caret_down,
    caret_left,
    caret_right,
    caret_square_down,
    caret_square_left,
    caret_square_right,
    caret_square_up,
    caret_up,
    carrot,
    cart_arrow_down,
    cart_plus,
    cash_register,
    cat,
    cc_amazon_pay,
    cc_amex,
    cc_apple_pay,
    cc_diners_club,
    cc_discover,
    cc_jcb,
    cc_mastercard,
    cc_paypal,
    cc_stripe,
    cc_visa,
    centercode,
    centos,
    certificate,
    chair,
    chalkboard,
    chalkboard_teacher,
    charging_station,
    chart_area,
    chart_bar,
    chart_line,
    chart_pie,
    check,
    check_circle,
    check_double,
    check_square,
    cheese,
    chess,
    chess_bishop,
    chess_board,
    chess_king,
    chess_knight,
    chess_pawn,
    chess_queen,
    chess_rook,
    chevron_circle_down,
    chevron_circle_left,
    chevron_circle_right,
    chevron_circle_up,
    chevron_down,
    chevron_left,
    chevron_right,
    chevron_up,
    child,
    chrome,
    chromecast,
    church,
    circle,
    circle_notch,
    city,
    clinic_medical,
    clipboard,
    clipboard_check,
    clipboard_list,
    clock,
    clone,
    closed_captioning,
    cloud,
    cloud_download_alt,
    cloud_meatball,
    cloud_moon,
    cloud_moon_rain,
    cloud_rain,
    cloud_showers_heavy,
    cloud_sun,
    cloud_sun_rain,
    cloud_upload_alt,
    cloudflare,
    cloudscale,
    cloudsmith,
    cloudversify,
    cocktail,
    code,
    code_branch,
    codepen,
    codiepie,
    coffee,
    cog,
    cogs,
    coins,
    columns,
    comment,
    comment_alt,
    comment_dollar,
    comment_dots,
    comment_medical,
    comment_slash,
    comments,
    comments_dollar,
    compact_disc,
    compass,
    compress,
    compress_alt,
    compress_arrows_alt,
    concierge_bell,
    confluence,
    connectdevelop,
    contao,
    cookie,
    cookie_bite,
    copy,
    copyright,
    cotton_bureau,
    couch,
    cpanel,
    creative_commons,
    creative_commons_by,
    creative_commons_nc,
    creative_commons_nc_eu,
    creative_commons_nc_jp,
    creative_commons_nd,
    creative_commons_pd,
    creative_commons_pd_alt,
    creative_commons_remix,
    creative_commons_sa,
    creative_commons_sampling,
    creative_commons_sampling_plus,
    creative_commons_share,
    creative_commons_zero,
    credit_card,
    critical_role,
    crop,
    crop_alt,
    cross,
    crosshairs,
    crow,
    crown,
    crutch,
    css3,
    css3_alt,
    cube,
    cubes,
    cut,
    cuttlefish,
    d_and_d,
    d_and_d_beyond,
    dailymotion,
    dashcube,
    database,
    deaf,
    deezer,
    delicious,
    democrat,
    deploydog,
    deskpro,
    desktop,
    dev,
    deviantart,
    dharmachakra,
    dhl,
    diagnoses,
    diaspora,
    dice,
    dice_d20,
    dice_d6,
    dice_five,
    dice_four,
    dice_one,
    dice_six,
    dice_three,
    dice_two,
    digg,
    digital_ocean,
    digital_tachograph,
    directions,
    discord,
    discourse,
    disease,
    divide,
    dizzy,
    dna,
    dochub,
    docker,
    dog,
    dollar_sign,
    dolly,
    dolly_flatbed,
    donate,
    door_closed,
    door_open,
    dot_circle,
    dove,
    download,
    draft2digital,
    drafting_compass,
    dragon,
    draw_polygon,
    dribbble,
    dribbble_square,
    dropbox,
    drum,
    drum_steelpan,
    drumstick_bite,
    drupal,
    dumbbell,
    dumpster,
    dumpster_fire,
    dungeon,
    dyalog,
    earlybirds,
    ebay,
    edge,
    edge_legacy,
    edit,
    egg,
    eject,
    elementor,
    ellipsis_h,
    ellipsis_v,
    ello,
    ember,
    empire,
    envelope,
    envelope_open,
    envelope_open_text,
    envelope_square,
    envira,
    equals,
    eraser,
    erlang,
    ethereum,
    ethernet,
    etsy,
    euro_sign,
    evernote,
    exchange_alt,
    exclamation,
    exclamation_circle,
    exclamation_triangle,
    expand,
    expand_alt,
    expand_arrows_alt,
    expeditedssl,
    external_link_alt,
    external_link_square_alt,
    eye,
    eye_dropper,
    eye_slash,
    facebook,
    facebook_f,
    facebook_messenger,
    facebook_square,
    fan,
    fantasy_flight_games,
    fast_backward,
    fast_forward,
    faucet,
    fax,
    feather,
    feather_alt,
    fedex,
    fedora,
    female,
    fighter_jet,
    figma,
    file,
    file_alt,
    file_archive,
    file_audio,
    file_code,
    file_contract,
    file_csv,
    file_download,
    file_excel,
    file_export,
    file_image,
    file_import,
    file_invoice,
    file_invoice_dollar,
    file_medical,
    file_medical_alt,
    file_pdf,
    file_powerpoint,
    file_prescription,
    file_signature,
    file_upload,
    file_video,
    file_word,
    fill,
    fill_drip,
    film,
    filter,
    fingerprint,
    fire,
    fire_alt,
    fire_extinguisher,
    firefox,
    firefox_browser,
    first_aid,
    first_order,
    first_order_alt,
    firstdraft,
    fish,
    fist_raised,
    flag,
    flag_checkered,
    flag_usa,
    flask,
    flickr,
    flipboard,
    flushed,
    fly,
    folder,
    folder_minus,
    folder_open,
    folder_plus,
    font,
    font_awesome,
    font_awesome_alt,
    font_awesome_flag,
    fonticons,
    fonticons_fi,
    football_ball,
    fort_awesome,
    fort_awesome_alt,
    forumbee,
    forward,
    foursquare,
    free_code_camp,
    freebsd,
    frog,
    frown,
    frown_open,
    fulcrum,
    funnel_dollar,
    futbol,
    galactic_republic,
    galactic_senate,
    gamepad,
    gas_pump,
    gavel,
    gem,
    genderless,
    get_pocket,
    gg,
    gg_circle,
    ghost,
    gift,
    gifts,
    git,
    git_alt,
    git_square,
    github,
    github_alt,
    github_square,
    gitkraken,
    gitlab,
    gitter,
    glass_cheers,
    glass_martini,
    glass_martini_alt,
    glass_whiskey,
    glasses,
    glide,
    glide_g,
    globe,
    globe_africa,
    globe_americas,
    globe_asia,
    globe_europe,
    gofore,
    golf_ball,
    goodreads,
    goodreads_g,
    google,
    google_drive,
    google_pay,
    google_play,
    google_plus,
    google_plus_g,
    google_plus_square,
    google_wallet,
    gopuram,
    graduation_cap,
    gratipay,
    grav,
    greater_than,
    greater_than_equal,
    grimace,
    grin,
    grin_alt,
    grin_beam,
    grin_beam_sweat,
    grin_hearts,
    grin_squint,
    grin_squint_tears,
    grin_stars,
    grin_tears,
    grin_tongue,
    grin_tongue_squint,
    grin_tongue_wink,
    grin_wink,
    grip_horizontal,
    grip_lines,
    grip_lines_vertical,
    grip_vertical,
    gripfire,
    grunt,
    guilded,
    guitar,
    gulp,
    h_square,
    hacker_news,
    hacker_news_square,
    hackerrank,
    hamburger,
    hammer,
    hamsa,
    hand_holding,
    hand_holding_heart,
    hand_holding_medical,
    hand_holding_usd,
    hand_holding_water,
    hand_lizard,
    hand_middle_finger,
    hand_paper,
    hand_peace,
    hand_point_down,
    hand_point_left,
    hand_point_right,
    hand_point_up,
    hand_pointer,
    hand_rock,
    hand_scissors,
    hand_sparkles,
    hand_spock,
    hands,
    hands_helping,
    hands_wash,
    handshake,
    handshake_alt_slash,
    handshake_slash,
    hanukiah,
    hard_hat,
    hashtag,
    hat_cowboy,
    hat_cowboy_side,
    hat_wizard,
    hdd,
    head_side_cough,
    head_side_cough_slash,
    head_side_mask,
    head_side_virus,
    heading,
    headphones,
    headphones_alt,
    headset,
    heart,
    heart_broken,
    heartbeat,
    helicopter,
    highlighter,
    hiking,
    hippo,
    hips,
    hire_a_helper,
    history,
    hive,
    hockey_puck,
    holly_berry,
    home,
    hooli,
    hornbill,
    horse,
    horse_head,
    hospital,
    hospital_alt,
    hospital_symbol,
    hospital_user,
    hot_tub,
    hotdog,
    hotel,
    hotjar,
    hourglass,
    hourglass_end,
    hourglass_half,
    hourglass_start,
    house_damage,
    house_user,
    houzz,
    hryvnia,
    html5,
    hubspot,
    i_cursor,
    ice_cream,
    icicles,
    icons,
    id_badge,
    id_card,
    id_card_alt,
    ideal,
    igloo,
    image,
    images,
    imdb,
    inbox,
    indent,
    index,
    industry,
    infinity,
    info,
    info_circle,
    innosoft,
    instagram,
    instagram_square,
    instalod,
    intercom,
    internet_explorer,
    invision,
    ioxhost,
    italic,
    itch_io,
    itunes,
    itunes_note,
    java,
    jedi,
    jedi_order,
    jenkins,
    jira,
    joget,
    joint,
    joomla,
    journal_whills,
    js,
    js_square,
    jsfiddle,
    kaaba,
    kaggle,
    key,
    keybase,
    keyboard,
    keycdn,
    khanda,
    kickstarter,
    kickstarter_k,
    kiss,
    kiss_beam,
    kiss_wink_heart,
    kiwi_bird,
    korvue,
    landmark,
    language,
    laptop,
    laptop_code,
    laptop_house,
    laptop_medical,
    laravel,
    lastfm,
    lastfm_square,
    laugh,
    laugh_beam,
    laugh_squint,
    laugh_wink,
    layer_group,
    leaf,
    leanpub,
    lemon,
    less,
    less_than,
    less_than_equal,
    level_down_alt,
    level_up_alt,
    life_ring,
    lightbulb,
    line,
    link,
    linkedin,
    linkedin_in,
    linode,
    linux,
    lira_sign,
    list,
    list_alt,
    list_ol,
    list_ul,
    location_arrow,
    lock,
    lock_open,
    long_arrow_alt_down,
    long_arrow_alt_left,
    long_arrow_alt_right,
    long_arrow_alt_up,
    low_vision,
    luggage_cart,
    lungs,
    lungs_virus,
    lyft,
    magento,
    magic,
    magnet,
    mail_bulk,
    mailchimp,
    male,
    mandalorian,
    map,
    map_marked,
    map_marked_alt,
    map_marker,
    map_marker_alt,
    map_pin,
    map_signs,
    markdown,
    marker,
    mars,
    mars_double,
    mars_stroke,
    mars_stroke_h,
    mars_stroke_v,
    mask,
    mastodon,
    maxcdn,
    mdb,
    medal,
    medapps,
    medium,
    medium_m,
    medkit,
    medrt,
    meetup,
    megaport,
    meh,
    meh_blank,
    meh_rolling_eyes,
    memory,
    mendeley,
    menorah,
    mercury,
    meteor,
    microblog,
    microchip,
    microphone,
    microphone_alt,
    microphone_alt_slash,
    microphone_slash,
    microscope,
    microsoft,
    minus,
    minus_circle,
    minus_square,
    mitten,
    mix,
    mixcloud,
    mixer,
    mizuni,
    mobile,
    mobile_alt,
    modx,
    monero,
    money_bill,
    money_bill_alt,
    money_bill_wave,
    money_bill_wave_alt,
    money_check,
    money_check_alt,
    monument,
    moon,
    mortar_pestle,
    mosque,
    motorcycle,
    mountain,
    mouse,
    mouse_pointer,
    mug_hot,
    music,
    napster,
    neos,
    network_wired,
    neuter,
    newspaper,
    nimblr,
    node,
    node_js,
    not_equal,
    notes_medical,
    npm,
    ns8,
    nutritionix,
    object_group,
    object_ungroup,
    octopus_deploy,
    odnoklassniki,
    odnoklassniki_square,
    oil_can,
    old_republic,
    om,
    opencart,
    openid,
    opera,
    optin_monster,
    orcid,
    osi,
    otter,
    outdent,
    page4,
    pagelines,
    pager,
    paint_brush,
    paint_roller,
    palette,
    palfed,
    pallet,
    paper_plane,
    paperclip,
    parachute_box,
    paragraph,
    parking,
    passport,
    pastafarianism,
    paste,
    patreon,
    pause,
    pause_circle,
    paw,
    paypal,
    peace,
    pen,
    pen_alt,
    pen_fancy,
    pen_nib,
    pen_square,
    pencil_alt,
    pencil_ruler,
    penny_arcade,
    people_arrows,
    people_carry,
    pepper_hot,
    perbyte,
    percent,
    percentage,
    periscope,
    person_booth,
    phabricator,
    phoenix_framework,
    phoenix_squadron,
    phone,
    phone_alt,
    phone_slash,
    phone_square,
    phone_square_alt,
    phone_volume,
    photo_video,
    php,
    pied_piper,
    pied_piper_alt,
    pied_piper_hat,
    pied_piper_pp,
    pied_piper_square,
    piggy_bank,
    pills,
    pinterest,
    pinterest_p,
    pinterest_square,
    pizza_slice,
    place_of_worship,
    plane,
    plane_arrival,
    plane_departure,
    plane_slash,
    play,
    play_circle,
    playstation,
    plug,
    plus,
    plus_circle,
    plus_square,
    podcast,
    poll,
    poll_h,
    poo,
    poo_storm,
    poop,
    portrait,
    pound_sign,
    power_off,
    pray,
    praying_hands,
    prescription,
    prescription_bottle,
    prescription_bottle_alt,
    print,
    procedures,
    product_hunt,
    project_diagram,
    pump_medical,
    pump_soap,
    pushed,
    puzzle_piece,
    python,
    qq,
    qrcode,
    question,
    question_circle,
    quidditch,
    quinscape,
    quora,
    quote_left,
    quote_right,
    quran,
    r_project,
    radiation,
    radiation_alt,
    rainbow,
    random,
    raspberry_pi,
    ravelry,
    react,
    reacteurope,
    readme,
    rebel,
    receipt,
    record_vinyl,
    recycle,
    red_river,
    reddit,
    reddit_alien,
    reddit_square,
    redhat,
    redo,
    redo_alt,
    registered,
    remove_format,
    renren,
    reply,
    reply_all,
    replyd,
    republican,
    researchgate,
    resolving,
    restroom,
    retweet,
    rev,
    ribbon,
    ring,
    road,
    robot,
    rocket,
    rocketchat,
    rockrms,
    route,
    rss,
    rss_square,
    ruble_sign,
    ruler,
    ruler_combined,
    ruler_horizontal,
    ruler_vertical,
    running,
    rupee_sign,
    rust,
    sad_cry,
    sad_tear,
    safari,
    salesforce,
    sass,
    satellite,
    satellite_dish,
    save,
    schlix,
    school,
    screwdriver,
    scribd,
    scroll,
    sd_card,
    search,
    search_dollar,
    search_location,
    search_minus,
    search_plus,
    searchengin,
    seedling,
    sellcast,
    sellsy,
    server,
    servicestack,
    shapes,
    share,
    share_alt,
    share_alt_square,
    share_square,
    shekel_sign,
    shield_alt,
    shield_virus,
    ship,
    shipping_fast,
    shirtsinbulk,
    shoe_prints,
    shopify,
    shopping_bag,
    shopping_basket,
    shopping_cart,
    shopware,
    shower,
    shuttle_van,
    sign,
    sign_in_alt,
    sign_language,
    sign_out_alt,
    signal,
    signature,
    sim_card,
    simplybuilt,
    sink,
    sistrix,
    sitemap,
    sith,
    skating,
    sketch,
    skiing,
    skiing_nordic,
    skull,
    skull_crossbones,
    skyatlas,
    skype,
    slack,
    slack_hash,
    slash,
    sleigh,
    sliders_h,
    slideshare,
    smile,
    smile_beam,
    smile_wink,
    smog,
    smoking,
    smoking_ban,
    sms,
    snapchat,
    snapchat_ghost,
    snapchat_square,
    snowboarding,
    snowflake,
    snowman,
    snowplow,
    soap,
    socks,
    solar_panel,
    sort,
    sort_alpha_down,
    sort_alpha_down_alt,
    sort_alpha_up,
    sort_alpha_up_alt,
    sort_amount_down,
    sort_amount_down_alt,
    sort_amount_up,
    sort_amount_up_alt,
    sort_down,
    sort_numeric_down,
    sort_numeric_down_alt,
    sort_numeric_up,
    sort_numeric_up_alt,
    sort_up,
    soundcloud,
    sourcetree,
    spa,
    space_shuttle,
    speakap,
    speaker_deck,
    spell_check,
    spider,
    spinner,
    splotch,
    spotify,
    spray_can,
    square,
    square_full,
    square_root_alt,
    squarespace,
    stack_exchange,
    stack_overflow,
    stackpath,
    stamp,
    star,
    star_and_crescent,
    star_half,
    star_half_alt,
    star_of_david,
    star_of_life,
    staylinked,
    steam,
    steam_square,
    steam_symbol,
    step_backward,
    step_forward,
    stethoscope,
    sticker_mule,
    sticky_note,
    stop,
    stop_circle,
    stopwatch,
    stopwatch_20,
    store,
    store_alt,
    store_alt_slash,
    store_slash,
    strava,
    stream,
    street_view,
    strikethrough,
    stripe,
    stripe_s,
    stroopwafel,
    studiovinari,
    stumbleupon,
    stumbleupon_circle,
    subscript,
    subway,
    suitcase,
    suitcase_rolling,
    sun,
    superpowers,
    superscript,
    supple,
    surprise,
    suse,
    swatchbook,
    swift,
    swimmer,
    swimming_pool,
    symfony,
    synagogue,
    sync,
    sync_alt,
    syringe,
    table,
    table_tennis,
    tablet,
    tablet_alt,
    tablets,
    tachometer_alt,
    tag,
    tags,
    tape,
    tasks,
    taxi,
    teamspeak,
    teeth,
    teeth_open,
    telegram,
    telegram_plane,
    temperature_high,
    temperature_low,
    tencent_weibo,
    tenge,
    terminal,
    text_height,
    text_width,
    th,
    th_large,
    th_list,
    the_red_yeti,
    theater_masks,
    themeco,
    themeisle,
    thermometer,
    thermometer_empty,
    thermometer_full,
    thermometer_half,
    thermometer_quarter,
    thermometer_three_quarters,
    think_peaks,
    thumbs_down,
    thumbs_up,
    thumbtack,
    ticket_alt,
    tiktok,
    times,
    times_circle,
    tint,
    tint_slash,
    tired,
    toggle_off,
    toggle_on,
    toilet,
    toilet_paper,
    toilet_paper_slash,
    toolbox,
    tools,
    tooth,
    torah,
    torii_gate,
    tractor,
    trade_federation,
    trademark,
    traffic_light,
    trailer,
    train,
    tram,
    transgender,
    transgender_alt,
    trash,
    trash_alt,
    trash_restore,
    trash_restore_alt,
    tree,
    trello,
    tripadvisor,
    trophy,
    truck,
    truck_loading,
    truck_monster,
    truck_moving,
    truck_pickup,
    tshirt,
    tty,
    tumblr,
    tumblr_square,
    tv,
    twitch,
    twitter,
    twitter_square,
    typo3,
    uber,
    ubuntu,
    uikit,
    umbraco,
    umbrella,
    umbrella_beach,
    uncharted,
    underline,
    undo,
    undo_alt,
    uniregistry,
    unity,
    universal_access,
    university,
    unlink,
    unlock,
    unlock_alt,
    unsplash,
    untappd,
    upload,
    ups,
    usb,
    user,
    user_alt,
    user_alt_slash,
    user_astronaut,
    user_check,
    user_circle,
    user_clock,
    user_cog,
    user_edit,
    user_friends,
    user_graduate,
    user_injured,
    user_lock,
    user_md,
    user_minus,
    user_ninja,
    user_nurse,
    user_plus,
    user_secret,
    user_shield,
    user_slash,
    user_tag,
    user_tie,
    user_times,
    users,
    users_cog,
    users_slash,
    usps,
    ussunnah,
    utensil_spoon,
    utensils,
    vaadin,
    vector_square,
    venus,
    venus_double,
    venus_mars,
    vest,
    vest_patches,
    viacoin,
    viadeo,
    viadeo_square,
    vial,
    vials,
    viber,
    video,
    video_slash,
    vihara,
    vimeo,
    vimeo_square,
    vimeo_v,
    vine,
    virus,
    virus_slash,
    viruses,
    vk,
    vnv,
    voicemail,
    volleyball_ball,
    volume_down,
    volume_mute,
    volume_off,
    volume_up,
    vote_yea,
    vr_cardboard,
    vuejs,
    walking,
    wallet,
    warehouse,
    watchman_monitoring,
    water,
    wave_square,
    waze,
    weebly,
    weibo,
    weight,
    weight_hanging,
    weixin,
    whatsapp,
    whatsapp_square,
    wheelchair,
    whmcs,
    wifi,
    wikipedia_w,
    wind,
    window_close,
    window_maximize,
    window_minimize,
    window_restore,
    windows,
    wine_bottle,
    wine_glass,
    wine_glass_alt,
    wix,
    wizards_of_the_coast,
    wodu,
    wolf_pack_battalion,
    won_sign,
    wordpress,
    wordpress_simple,
    wpbeginner,
    wpexplorer,
    wpforms,
    wpressr,
    wrench,
    x_ray,
    xbox,
    xing,
    xing_square,
    y_combinator,
    yahoo,
    yammer,
    yandex,
    yandex_international,
    yarn,
    yelp,
    yen_sign,
    yin_yang,
    yoast,
    youtube,
    youtube_square,
    zhihu,
}