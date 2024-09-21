use tux_icons::icon_fetcher::IconFetcher;

fn main(){
    let fetcher = IconFetcher::new().set_return_target_path(true);
    let path = fetcher.get_icon_path_from_desktop("/usr/share/applications/nwg-look.desktop");
    
    println!("Path: {:?}", path);
}