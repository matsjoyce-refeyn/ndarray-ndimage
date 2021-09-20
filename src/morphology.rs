use ndarray::{s, Zip};

use crate::{dim_minus_1, Kernel3d, Mask};

/// Binary erosion of a 3D image.
pub fn binary_erosion(mask: &Mask, kernel: Kernel3d) -> Mask {
    // By definition, all borders are set to 0
    let mut eroded_mask = mask.clone();
    let (width, height, depth) = dim_minus_1(mask);
    eroded_mask.slice_mut(s![0, .., ..]).fill(false);
    eroded_mask.slice_mut(s![width, .., ..]).fill(false);
    eroded_mask.slice_mut(s![.., 0, ..]).fill(false);
    eroded_mask.slice_mut(s![.., height, ..]).fill(false);
    eroded_mask.slice_mut(s![.., .., 0]).fill(false);
    eroded_mask.slice_mut(s![.., .., depth]).fill(false);

    // Erode the mask when at least one of the values doesn't respect the kernel.
    // An erosion is defined either as `all(!(!w & k))` or `!any(!w & k)`.
    // Note that an empty kernel will always produce a full mask.
    let zone = eroded_mask.slice_mut(s![1..width, 1..height, 1..depth]);
    if kernel == Kernel3d::Full {
        Zip::from(mask.windows((3, 3, 3))).map_assign_into(zone, |w| !w.iter().any(|w| !w));
    } else {
        Zip::from(mask.windows((3, 3, 3))).map_assign_into(zone, |w| {
            // This ugly condition is equivalent to
            // *mask = !w.iter().zip(&kernel).any(|(w, k)| !w & k)
            // but it's around 5x faster because there's no branch misprediction
            !(!w[(0, 1, 1)]
                || !w[(1, 0, 1)]
                || !w[(1, 1, 0)]
                || !w[(1, 1, 1)]
                || !w[(1, 1, 2)]
                || !w[(1, 2, 1)]
                || !w[(2, 1, 1)])
        });
    }
    eroded_mask
}
