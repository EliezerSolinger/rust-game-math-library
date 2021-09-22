#![allow(dead_code,non_snake_case)]
const PI : f32 = 3.1415;
const HALF_PI : f32 = PI / 2.0;
const DOUBLE_PI : f32 = PI * 2.0;
const EQUAL_EPSILON : f32 = f32::EPSILON;

pub fn float_equals(a:f32,b:f32)->bool {((a - EQUAL_EPSILON) < b) && (b <( a + EQUAL_EPSILON))}


pub fn deg2rad(a:f32)->f32 { ( (a) * PI ) / 180.0 }
pub fn rad2deg(a:f32)->f32 { ( (a) * 180.0 ) / PI }
pub fn lerp(a:f32,b:f32,d:f32)->f32 {((1.0-d)*a) + (d*b)}
pub fn fast_lerp(a:f32,b:f32,t:f32)->f32 {a + t * (b - a)}

pub fn cos_from_sin(sin:f32,angle:f32)->f32 {
    let cos= (1.0 - sin*sin).sqrt();
    let a = angle + HALF_PI;
    let  b = a- (a / DOUBLE_PI).floor() * DOUBLE_PI;
    if b < 0.0 && DOUBLE_PI + b >= PI {
        -cos
    } else {
        cos
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r:f32,
    pub g:f32,
    pub b:f32,
    pub a:f32
}


impl Color {
    const WHITE:Color = Color{r:1.,g:1.,b:1.,a:1.};
    const BLACK:Color = Color{r:1.,g:1.,b:1.,a:1.};
    const RED:Color = Color{r:1.,g:1.,b:1.,a:1.};
    const GREEN:Color = Color{r:1.,g:1.,b:1.,a:1.};
    const TRANSPARENT:Color = Color{r:1.,g:1.,b:1.,a:0.};

    pub fn from_hex_rgba(hex: u32)->Color {
        Color {
            r: (((hex>>24) as u8) as f32)/255.0,
            g: (((hex>>16) as u8) as f32)/255.0,
            b: (((hex>>8) as u8) as f32)/255.0,
            a: (((hex>>0) as u8) as f32)/255.0,
        }
    }
    pub fn from_hex_rgb(hex: u32)->Color {
        Color {
            r: (((hex>>16) as u8) as f32)/255.0,
            g: (((hex>>8) as u8) as f32)/255.0,
            b: (((hex>>0) as u8) as f32)/255.0,
            a: 1.0,
        }
    }
    pub fn lerp(a:Color,b:Color,t:f32)->Color {
        Color {
             r: lerp(a.r,b.r,t),
             g: lerp(a.g,b.g,t),
             b: lerp(a.b,b.b,t),
             a: lerp(a.a,b.a,t)
        }
    }

}
/* two dimensional vector */
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x:f32,
    pub y:f32
}

impl Vec2 {
    const ZERO:Vec2 = Vec2{x:0.,y:0.};
    const FILL_ONE:Vec2 = Vec2{x:1.,y:1.};
    const UP:Vec2 = Vec2{x:0.,y:1.};
    const DOWN:Vec2 = Vec2{x:0.,y:-1.};
    const LEFT:Vec2 = Vec2{x:-1.,y:0.};
    const RIGHT:Vec2 = Vec2{x:1.,y:0.};    
    pub fn add(&self,other:Vec2)->Vec2{
        Vec2 { x:self.x + other.x, y:self.y + other.y }
    }
    pub fn sub(&self,other:Vec2)->Vec2{
        Vec2 { x: self.x - other.x, y: self.y - other.y}
    }
    pub fn mul(&self,other:Vec2)->Vec2{
        Vec2 { x: self.x * other.x, y:self.y * other.y}
    }
    pub fn div(&self,other:Vec2)->Vec2 {
        Vec2 { x: self.x / other.x, y:self.y / other.y}
    }
    pub fn dot(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    pub fn abs(&self) -> Vec2 {
        Vec2{x: self.x.abs(), y: self.y.abs()}
    }
    pub fn length(&self) ->f32 {
        self.dot().sqrt()
    }
    pub fn magnitude(&self) -> f32 {
        self.length()
    }
    pub fn normalized(&self) -> Vec2 {
        let length=self.length();
        Vec2 { x:self.x / length, y:self.y / length}
    }
    pub fn distance(a: Vec2, b: Vec2) -> f32 {
        b.sub(a).magnitude()
    }
    pub fn scale(&self,v:f32)->Vec2 {
        Vec2 { x: self.x*v, y:self.y*v}
    }
    pub fn lerp(a:Vec2,b:Vec2,t:f32) ->Vec2 {
        Vec2 {
             x: lerp(a.x,b.x,t),
             y: lerp(a.y,b.y,t)
        }
    }
    pub fn invert(&self) ->Vec2 {
        self.scale(-1.)
    }
}

pub struct Rect {
    pub size:Vec2,
    pub position:Vec2
}
impl Rect {
    pub fn empty() -> Rect {
        Rect {
            size: Vec2::FILL_ONE,
            position:Vec2::ZERO
        } 
    }
}

#[derive(Copy, Clone)]
pub struct Circle {
    pub position:Vec2,
    pub radius:f32
}
impl Circle {
    const EMPTY:Circle = Circle{position: Vec2::ZERO, radius:1.0};
    pub fn new(radius:f32) -> Circle {
        Circle {
            position:Vec2::ZERO,
            radius:radius
        }
    }
}


#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x:f32,
    pub y:f32,
    pub z:f32
}

impl Vec3 {
    const ZERO:Vec3 = Vec3{x:0.,y:0.,z:0.};
    const FILL_ONE:Vec3 = Vec3{x:1.,y:1.,z:1.};
    const UP:Vec3 = Vec3{x:0.,y:1.0,z:0.};
    const DOWN:Vec3 = Vec3{x:0.,y:-1.,z:0.};
    const LEFT:Vec3 = Vec3{x:-1.,y:0.,z:0.};
    const RIGHT:Vec3 = Vec3{x:1.,y:0.,z:0.};
    const FORWARD:Vec3 = Vec3{x:0.,y:0.,z:1.};
    const BACKWARD:Vec3 = Vec3{x:0.,y:0.,z:-1.};
    
    pub fn add(&self,other:Vec3)->Vec3{
        Vec3{ x:self.x + other.x, y:self.y + other.y, z:self.z + other.z}
    } 
    pub fn sub(&self,other:Vec3)->Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z:self.z - other.z}
    }
    pub fn mul(&self,other:Vec3)->Vec3 {
        Vec3 { x: self.x * other.x, y:self.y * other.y, z:self.z * other.z}
    }
    pub fn div(&self,other:Vec3)->Vec3 {
        Vec3 { x: self.x / other.x, y: self.y / other.y,z:self.z / other.z}
    }
    pub fn dot(&self) -> f32 {
        self.x*self.x + self.y* self.y + self.z*self.z
    }
    pub fn abs(&self) -> Vec3 {
        Vec3 { x: self.x.abs(),y:self.y.abs(),z:self.z.abs()}
    }
    pub fn length(&self)-> f32 {
        self.dot().sqrt()
    }
    pub fn magnitude(&self)-> f32 {
        self.length()
    }
    pub fn normalized(&self)->Vec3 {
        let length=self.length();
        Vec3{ x: self.x / length, y:self.y / length,z:self.z/length}
    }
    pub fn distance(a:Vec3,b:Vec3) {
        b.sub(a).magnitude();
    }
    pub fn scale(&self,v:f32)->Vec3 {
        Vec3 { x: self.x*v, y: self.y*v, z: self.z*v }
    }
    pub fn lerp(a:Vec3,b:Vec3,t:f32)->Vec3 {
        Vec3 {
             x: lerp(a.x,b.x,t),
             y: lerp(a.y,b.y,t),
             z: lerp(a.y,b.y,t)
        }
    }
    pub fn invert(&self) ->Vec3 {
        self.scale(-1.)
    }
}

pub struct Box {
    pub size:Vec3,
    pub position:Vec3
}
impl Box {
    const EMPTY:Box = Box{
        size:Vec3::FILL_ONE, 
        position:Vec3::ZERO
    };
}

#[derive(Copy, Clone)]
pub struct Matrix4 {
    pub data:[[f32;4];4]
}
impl Matrix4 {
    const EMPTY:Matrix4 = Matrix4 {
        data:[
            [0.,0.,0.,0.],
            [0.,0.,0.,0.],
            [0.,0.,0.,0.],
            [0.,0.,0.,0.]
        ]
    };
    const IDENTITY:Matrix4 = Matrix4 {
        data:[
            [1.,0.,0.,0.],
            [0.,1.,0.,0.],
            [0.,0.,1.,0.],
            [0.,0.,0.,1.]
        ]
    };
    /**
     * @brief Creates a matrix for a symetric perspective-view frustum based on
     *        the default handedness.
     * @param fovy Specifies the field of view angle in the y direction.
     *             Expressed in radians.
     * @param aspect Specifies the aspect ratio that determines the field of view
     *               in the x direction. The aspect ratio is the ratio of
     *               x (width) to y (height).
     * @param near Specifies the distance from the viewer to the near clipping
     *             plane (always positive).
     * @param far Specifies the distance from the viewer to the far clipping
     *            plane (always positive).
     */
    pub fn perspective(fovy:f32,aspect:f32,znear:f32,zfar:f32)->Matrix4 {
            
        let tan_half_fovy:f32 = (fovy /  2.0).tan();

        let mut result = Matrix4::EMPTY;
        result.data[0][0] = 1.0 / (aspect * tan_half_fovy);
        result.data[1][1] = 1.0 / (tan_half_fovy);
        result.data[2][2] = -(zfar + znear) / (zfar - znear);
        result.data[2][3] = -1.;
        result.data[3][2] = -( 2. * zfar * znear) / (zfar - znear);
        result
    }
    /**
     * @brief Creates a matrix for an orthographic parallel viewing volume.
     */
    pub fn ortho(left:f32,right:f32,bottom:f32,top:f32,znear:f32,zfar:f32)->Matrix4 {
        let mut result = Matrix4::IDENTITY;
        result.data[0][0] =  2. / (right - left);
        result.data[1][1] =  2. / (top - bottom);
        result.data[2][2] = -2. / (zfar - znear);
        result.data[3][0] = -(right + left) / (right - left);
        result.data[3][1] = -(top + bottom) / (top - bottom);
        result.data[3][2] = -(zfar + znear) / (zfar - znear);
        result
    }

    pub fn frustum(left:f32,right:f32,bottom:f32,top:f32,znear:f32,zfar:f32)->Matrix4 {
        let mut result = Matrix4::EMPTY;
        result.data[0][0] = ( 2. * znear) / (right - left);
        result.data[1][1] = (2. * znear) / (top - bottom);
        result.data[2][0] = (right + left) / (right - left);
        result.data[2][1] = (top + bottom) / (top - bottom);
        result.data[2][2] = -(zfar + znear) / (zfar - znear);
        result.data[2][3] = -1.;
        result.data[3][2] = -(2. * zfar * znear) / (zfar - znear);
        result
    }

    pub fn mul(&mut self,other:&Matrix4) {
        self.data=[
            [
                    other.data[0][0] * self.data[0][0] + other.data[0][1] * self.data[1][0] + other.data[0][2] * self.data[2][0] + other.data[0][3] * self.data[3][0] , 
                    other.data[0][0] * self.data[0][1] + other.data[0][1] * self.data[1][1] + other.data[0][2] * self.data[2][1] + other.data[0][3] * self.data[3][1] , 
                    other.data[0][0] * self.data[0][2] + other.data[0][1] * self.data[1][2] + other.data[0][2] * self.data[2][2] + other.data[0][3] * self.data[3][2] , 
                    other.data[0][0] * self.data[0][3] + other.data[0][1] * self.data[1][3] + other.data[0][2] * self.data[2][3] + other.data[0][3] * self.data[3][3] 
                ], 
            [
                    other.data[1][0] * self.data[0][0] + other.data[1][1] * self.data[1][0] + other.data[1][2] * self.data[2][0] + other.data[1][3] * self.data[3][0] , 
                    other.data[1][0] * self.data[0][1] + other.data[1][1] * self.data[1][1] + other.data[1][2] * self.data[2][1] + other.data[1][3] * self.data[3][1] , 
                    other.data[1][0] * self.data[0][2] + other.data[1][1] * self.data[1][2] + other.data[1][2] * self.data[2][2] + other.data[1][3] * self.data[3][2] , 
                    other.data[1][0] * self.data[0][3] + other.data[1][1] * self.data[1][3] + other.data[1][2] * self.data[2][3] + other.data[1][3] * self.data[3][3] 
                ],
            [
                other.data[2][0] * self.data[0][0] + other.data[2][1] * self.data[1][0] + other.data[2][2] * self.data[2][0] + other.data[2][3] * self.data[3][0],
                other.data[2][0] * self.data[0][1] + other.data[2][1] * self.data[1][1] + other.data[2][2] * self.data[2][1] + other.data[2][3] * self.data[3][1] , 
                other.data[2][0] * self.data[0][2] + other.data[2][1] * self.data[1][2] + other.data[2][2] * self.data[2][2] + other.data[2][3] * self.data[3][2] , 
                other.data[2][0] * self.data[0][3] + other.data[2][1] * self.data[1][3] + other.data[2][2] * self.data[2][3] + other.data[2][3] * self.data[3][3] 
                ], 
    
            [
                other.data[3][0] * self.data[0][0] + other.data[3][1] * self.data[1][0] + other.data[3][2] * self.data[2][0] + other.data[3][3] * self.data[3][0], 
                other.data[3][0] * self.data[0][1] + other.data[3][1] * self.data[1][1] + other.data[3][2] * self.data[2][1] + other.data[3][3] * self.data[3][1] , 
                other.data[3][0] * self.data[0][2] + other.data[3][1] * self.data[1][2] + other.data[3][2] * self.data[2][2] + other.data[3][3] * self.data[3][2] , 
                other.data[3][0] * self.data[0][3] + other.data[3][1] * self.data[1][3] + other.data[3][2] * self.data[2][3] + other.data[3][3] * self.data[3][3] 
            ] 
        ]
    }
    pub fn multiplicated(&self,m2:&Matrix4) ->Matrix4 {
        let mut result=self.clone();
        result.mul(m2);
        result
    }
    
    pub fn translate2D(&mut self,translaction:Vec2) {
        self.data[0][0]*=translaction.x;
        self.data[1][1]*=translaction.x;
    }
    pub fn translated2D(&self,translaction:Vec2)->Matrix4 {
        let mut new_matrix:Matrix4=self.clone();
        new_matrix.translate2D(translaction);
        new_matrix
    }
    pub fn translate(&mut self,translaction:Vec3) {
        self.data[0][0]*=translaction.x;
        self.data[1][1]*=translaction.x;
        self.data[2][2]*=translaction.x;
    }
    pub fn translated(&self,translaction:Vec3)->Matrix4 {
        let mut new_matrix:Matrix4=self.clone();
        new_matrix.translate(translaction);
        new_matrix
    }
    pub fn rotation_matrix(euler_angles:Vec3)->Matrix4 {
        let sin_x = euler_angles.x.sin();
        let cos_x = cos_from_sin(sin_x,euler_angles.x);
        let sin_y =  euler_angles.y.sin();
        let cos_y =  cos_from_sin(sin_y,euler_angles.y);
        let sin_z =  euler_angles.z.sin();
        let cos_z =  cos_from_sin(sin_z,euler_angles.z);
        let m_sin_x = -sin_x;
        let m_sin_y = -sin_y;
        let m_sin_z = -sin_z;
    
        // rotateX
        let nm11 = cos_x;
        let nm12 = sin_x;
        let nm21 = m_sin_x;
        let nm22 = cos_x;
        // rotateY
        let nm00 = cos_y;
        let nm01 = nm21 * m_sin_y;
        let nm02 = nm22 * m_sin_y;
        let mut new_matrix=Matrix4::IDENTITY;

        new_matrix.data[0][2] = sin_y;
        new_matrix.data[1][2] = nm21 * cos_y;
        new_matrix.data[2][2] = nm22 * cos_y;
        // rotateZ
        new_matrix.data[0][0] = nm00 * cos_z;
        new_matrix.data[1][0] = nm01 * cos_z + nm11 * sin_z;
        new_matrix.data[2][0] = nm02 * cos_z + nm12 * sin_z;
        new_matrix.data[0][1] = nm00 * m_sin_z;
        new_matrix.data[1][1] = nm01 * m_sin_z + nm11 * cos_z;
        new_matrix.data[2][1] = nm02 * m_sin_z + nm12 * cos_z;
        new_matrix
    }
    pub fn rotated(&self,rotation:Vec3)->Matrix4 {
        Matrix4::rotation_matrix(rotation).multiplicated(self)
    }
    pub fn rotate(&mut self,rotation:Vec3) {
        self.data=self.rotated(rotation).data;
    }
    pub fn rotated2D(&self,rotation:f32)->Matrix4 {
        self.rotated(Vec3{x:0.,y:0.,z:rotation})
    }
    pub fn rotate2D(&mut self,rotation:f32) {
        self.rotate(Vec3{x:0.,y:0.,z:rotation})
    }
    
    pub fn scale(&mut self,scale:Vec3) {  
        self.data[0][0]*=scale.x;
        self.data[1][1]*=scale.y;
        self.data[2][2]*=scale.z;
    }
    pub fn scaled(&self,scale:Vec3)->Matrix4 {
        let mut new_matrix=self.clone();
        new_matrix.scale(scale);
        new_matrix
    }
    pub fn scale2D(&mut self,scale:Vec2) {  
        self.data[0][0]*=scale.x;
        self.data[1][1]*=scale.y;
    }
    pub fn scaled2D(&self,scale:Vec2)->Matrix4 {
        let mut new_matrix=self.clone();
        new_matrix.scale2D(scale);
        new_matrix
    }
}
  

/*   TRANSFORMATIONS  */

pub struct Transform2D {
    pub position:Vec2,
    pub size:Vec2,
    pub rotation:f32
}

impl Transform2D {
    const IDENTITY:Transform2D=Transform2D{
        position:Vec2::ZERO,
        size:Vec2::FILL_ONE,
        rotation:0.
    };
    pub fn into_matrix(&self,matrix:&mut Matrix4) {
        *matrix=Matrix4::IDENTITY;
        matrix.scale2D(self.size);
        matrix.rotate2D(self.rotation);
        matrix.translate2D(self.position);
    }
    pub fn to_matrix(&self)->Matrix4 {
        let mut new_matrix=Matrix4::IDENTITY;
        self.into_matrix(&mut new_matrix);
        new_matrix
    }
    /*
    * view matrix is the relationship between world and camera
    * then, is the inverse process of model matrix
    *  */
    pub fn into_view_matrix(&self,matrix:&mut Matrix4) {
        *matrix=Matrix4::IDENTITY;
        matrix.translate2D(self.position.invert());
        matrix.rotate2D(self.rotation);
        matrix.scale2D(self.size);
    }
    pub fn to_view_matrix(&self)->Matrix4 {
        let mut new_matrix=Matrix4::IDENTITY;
        self.into_view_matrix(&mut new_matrix);
        new_matrix
    }
}



/*   TRANSFORMATIONS  */

pub struct Transform3D {
    pub position:Vec3,
    pub size:Vec3,
    pub euler_angles:Vec3
}

impl Transform3D {
    const IDENTITY:Transform3D=Transform3D{
        position:Vec3::ZERO,
        size:Vec3::FILL_ONE,
        euler_angles:Vec3::ZERO
    };
    pub fn into_matrix(&self,matrix:&mut Matrix4) {
        *matrix=Matrix4::IDENTITY;
        matrix.scale(self.size);
        matrix.rotate(self.euler_angles);
        matrix.translate(self.position);
    }
    pub fn to_matrix(&self)->Matrix4 {
        let mut new_matrix=Matrix4::IDENTITY;
        self.into_matrix(&mut new_matrix);
        new_matrix
    }
    /*
    * view matrix is the relationship between world and camera
    * then, is the inverse process of model matrix
    *  */
    pub fn into_view_matrix(&self,matrix:&mut Matrix4) {
        *matrix=Matrix4::IDENTITY;
        matrix.translate(self.position.invert());
        matrix.rotate(self.euler_angles);
        matrix.scale(self.size);
    }
    pub fn to_view_matrix(&self)->Matrix4 {
        let mut new_matrix=Matrix4::IDENTITY;
        self.into_view_matrix(&mut new_matrix);
        new_matrix
    }
}

