use crate::geometry::Point;

cpp_class!(
    /// A wrapper around the dlib `full_object_detection` class. Its internal
    /// storage is `std::vector<point>` on dlib < 20.0.1 and `std::vector<dpoint>`
    /// on dlib >= 20.0.1.
    /// https://github.com/davisking/dlib/blob/master/dlib/image_processing/full_object_detection.h#L21
    pub unsafe struct FaceLandmarks as "dlib::full_object_detection"
);

impl FaceLandmarks {
    /// Copy the landmark points out of the underlying dlib object.
    ///
    /// We copy via `part(i)` rather than reinterpret-casting the internal
    /// storage as `*const Point`: dlib 20.0.1 changed the storage from
    /// `std::vector<point>` (two `long`s) to `std::vector<dpoint>` (two
    /// `double`s), so the old cast would read garbage on 20.0.1+. Both
    /// `point` and `dpoint` expose `.x()`/`.y()`, and the `(long)` cast
    /// narrows `double → long` on the newer storage.
    pub fn points(&self) -> Vec<Point> {
        let len = unsafe {
            cpp!([self as "const dlib::full_object_detection*"] -> usize as "size_t" {
                return self->num_parts();
            })
        };

        let mut points: Vec<Point> = vec![Point::default(); len];
        if len > 0 {
            let out = points.as_mut_ptr();
            unsafe {
                cpp!([
                    self as "const dlib::full_object_detection*",
                    out as "long*",
                    len as "size_t"
                ] {
                    for (size_t i = 0; i < len; i++) {
                        auto p = self->part(i);
                        out[2 * i]     = static_cast<long>(p.x());
                        out[2 * i + 1] = static_cast<long>(p.y());
                    }
                });
            }
        }

        points
    }
}

#[test]
fn test_default_landmarks() {
    // ensure that FaceLandmarks::default() doesnt allow memory violations in safe code
    let landmarks = FaceLandmarks::default();
    let points = landmarks.points();

    assert!(points.is_empty());
    assert_eq!(points.len(), 0);
    assert_eq!(points.get(0), None);
}
