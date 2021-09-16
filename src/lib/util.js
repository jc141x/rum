const bannerBase = 'https://bkftwbhopivmrgzcagus.supabase.in/storage/v1/object/public/banners/';

export const getBanner = (game) => {
  return game.banner_index !== null
    ? `${bannerBase}${game.hash}-${game.banner_index}.png`
    : `${bannerBase}${game.hash}.png`;
};
