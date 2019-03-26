use approxeq::ApproxEq;
use num_traits::Float;
use trig::Trig;
use {TypedRotation3D, TypedTransform3D, TypedVector3D, UnknownUnit};

/// A rigid transformation. All lengths are preserved under such a transformation.
///
///
/// Internally, this is a rotation and a translation, with the rotation
/// applied first (i.e. `Translation * Rotation`)
///
/// This can be more efficient to use over full matrices, especially if you
/// have to deal with the decomposed quantities often.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct TypedRigidTransform3D<T, U> {
    pub rotation: TypedRotation3D<T, U, U>,
    pub translation: TypedVector3D<T, U>,
}

pub type RigidTransform3D<T> = TypedRigidTransform3D<T, UnknownUnit>;

impl<T: Float + ApproxEq<T>, U> TypedRigidTransform3D<T, U> {
    /// Construct a new rigid transformation, where the `rotation` applies first
    pub fn new(rotation: TypedRotation3D<T, U, U>, translation: TypedVector3D<T, U>) -> Self {
        Self {
            rotation,
            translation,
        }
    }

    /// Construct an identity transform
    pub fn identity() -> Self {
        Self {
            rotation: TypedRotation3D::identity(),
            translation: TypedVector3D::zero(),
        }
    }

    /// Construct a new rigid transformation, where the `translation` applies first
    pub fn new_from_reversed(
        translation: TypedVector3D<T, U>,
        rotation: TypedRotation3D<T, U, U>,
    ) -> Self {
        (Self {
            rotation,
            translation,
        })
        .reverse()
    }

    pub fn from_rotation(rotation: TypedRotation3D<T, U, U>) -> Self {
        Self {
            rotation,
            translation: TypedVector3D::zero(),
        }
    }

    pub fn from_translation(translation: TypedVector3D<T, U>) -> Self {
        Self {
            translation,
            rotation: TypedRotation3D::identity(),
        }
    }

    /// Provide the equivalent rigid transformation obtained by applying self's translation first
    /// and rotation second
    pub fn reverse(&self) -> Self {
        // R * T
        //   = R * T * (R^-1 * R)
        //   = (R * T * R^-1) * R
        //   = T' * R
        //
        // T' = (R * T * R^-1) is also a translation matrix
        // It is equivalent to the translation matrix obtained by rotating the
        // translation by R

        let translation = self.rotation.rotate_vector3d(&self.translation);
        Self {
            rotation: self.rotation,
            translation,
        }
    }

    /// Decompose this into a position and an orientation to be applied in the opposite order
    pub fn decompose_reversed(&self) -> (TypedVector3D<T, U>, TypedRotation3D<T, U, U>) {
        // self = T * R
        //      = R * R^-1 * T * R
        //      = R * (R^-1 * T * R)
        //      = R * T'
        //
        // T' = (R^-1 * T * R) is T rotated by R^-1

        let translation = self.rotation.inverse().rotate_vector3d(&self.translation);
        (translation, self.rotation)
    }

    /// Returns the multiplication of the two transforms such that
    /// other's transformation applies after self's transformation.
    ///
    /// i.e., this produces `other * self`, postmultiplying self to other
    pub fn post_mul(&self, other: &Self) -> Self {
        // self = T1 * R1
        // other = T2 * R2
        // result = T2 * R2 * T1 * R1
        //        = T2 * R2 * R1 * R1^-1 * T1 * R1
        //        = T2 * R2 * R1 * (R1^-1 * T1 * R1)
        //        = T2 * R' * T'
        //        = T2 * R' * T' * R'^-1 * R'
        //        = T2 * (R' * T' * R'^-1) * R'
        //        = T2 * T'' * R'
        //        = T'' * R'
        //
        // (R1^-1 * T1 * R1) = T' = T1 rotated by R1^-1
        // R2 * R1  = R'
        // (R' * T' * R'^-1) = T'' = T' rotated by R'
        // T2 * T'' = T''' = T2 + T'

        let t_prime = self.rotation.inverse().rotate_vector3d(&self.translation);
        let r_prime = self.rotation.post_rotate(&other.rotation);
        let t_prime2 = r_prime.rotate_vector3d(&t_prime);
        let t_prime3 = t_prime2 + other.translation;
        Self {
            rotation: r_prime,
            translation: t_prime3,
        }
    }

    /// Returns the multiplication of the two transforms such that
    /// self's transformation applies after other's transformation.
    ///
    /// i.e., this produces `self * other`, premultiplying self to other
    pub fn pre_mul(&self, other: &Self) -> Self {
        other.post_mul(&self)
    }

    /// Inverts the transformation
    pub fn inverse(&self) -> Self {
        // result = (self)^-1
        //        = (T * R)^-1
        //        = R^-1 * T^-1
        //        = R^-1 * T^-1 * R * R^-1
        //        = (R^-1 * T^-1 * R) * R^-1
        //        = T' * R'
        //
        // T' = (R^-1 * T^-1 * R) = (-T) rotated by R^-1
        // R' = R^-1
        //
        // An easier way of writing this is to use new_from_reversed() with R^-1 and T^-1

        Self::new_from_reversed(-self.translation, self.rotation.inverse())
    }

    pub fn to_transform(&self) -> TypedTransform3D<T, U, U>
    where
        T: Trig,
    {
        self.translation
            .to_transform()
            .pre_mul(&self.rotation.to_transform())
    }
}

impl<T: Float + ApproxEq<T>, U> From<TypedRotation3D<T, U, U>> for TypedRigidTransform3D<T, U> {
    fn from(rot: TypedRotation3D<T, U, U>) -> Self {
        Self::from_rotation(rot)
    }
}

impl<T: Float + ApproxEq<T>, U> From<TypedVector3D<T, U>> for TypedRigidTransform3D<T, U> {
    fn from(t: TypedVector3D<T, U>) -> Self {
        Self::from_translation(t)
    }
}

#[cfg(test)]
mod test {
    use super::RigidTransform3D;
    use {Rotation3D, TypedTransform3D, Vector3D};

    #[test]
    fn test_rigid_construction() {
        let translation = Vector3D::new(12.1, 17.8, -5.5);
        let rotation = Rotation3D::unit_quaternion(0.5, -7.8, 2.2, 4.3);

        let rigid = RigidTransform3D::new(rotation, translation);
        assert!(rigid
            .to_transform()
            .approx_eq(&translation.to_transform().pre_mul(&rotation.to_transform())));

        let rigid = RigidTransform3D::new_from_reversed(translation, rotation);
        assert!(rigid.to_transform().approx_eq(
            &translation
                .to_transform()
                .post_mul(&rotation.to_transform())
        ));
    }

    #[test]
    fn test_rigid_decomposition() {
        let translation = Vector3D::new(12.1, 17.8, -5.5);
        let rotation = Rotation3D::unit_quaternion(0.5, -7.8, 2.2, 4.3);

        let rigid = RigidTransform3D::new(rotation, translation);
        let (t2, r2) = rigid.decompose_reversed();
        assert!(rigid
            .to_transform()
            .approx_eq(&t2.to_transform().post_mul(&r2.to_transform())));
    }

    #[test]
    fn test_rigid_inverse() {
        let translation = Vector3D::new(12.1, 17.8, -5.5);
        let rotation = Rotation3D::unit_quaternion(0.5, -7.8, 2.2, 4.3);

        let rigid = RigidTransform3D::new(rotation, translation);
        let inverse = rigid.inverse();
        assert!(rigid
            .post_mul(&inverse)
            .to_transform()
            .approx_eq(&TypedTransform3D::identity()));
        assert!(inverse
            .to_transform()
            .approx_eq(&rigid.to_transform().inverse().unwrap()));
    }

    #[test]
    fn test_rigid_multiply() {
        let translation = Vector3D::new(12.1, 17.8, -5.5);
        let rotation = Rotation3D::unit_quaternion(0.5, -7.8, 2.2, 4.3);
        let translation2 = Vector3D::new(9.3, -3.9, 1.1);
        let rotation2 = Rotation3D::unit_quaternion(0.1, 0.2, 0.3, -0.4);
        let rigid = RigidTransform3D::new(rotation, translation);
        let rigid2 = RigidTransform3D::new(rotation2, translation2);

        assert!(rigid
            .post_mul(&rigid2)
            .to_transform()
            .approx_eq(&rigid.to_transform().post_mul(&rigid2.to_transform())));
        assert!(rigid
            .pre_mul(&rigid2)
            .to_transform()
            .approx_eq(&rigid.to_transform().pre_mul(&rigid2.to_transform())));
    }
}
