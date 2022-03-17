/*
MIT License
Copyright (c) 2021 Germán Molina
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::rand::*;
use crate::colour::Spectrum;
use crate::Float;
use crate::scene::Scene;
use crate::camera::Camera;

use crate::backward_metropolis::path::Path;

pub trait Mutation<'a> {    
    fn mutate(&self, _x: &Path,  scene: &'a Scene, camera: &dyn Camera, min_length: usize, rng: &mut SmallRng) -> Path<'a>;
}



/// A set of mutations and their respective probabilities.
#[derive(Default)]
pub struct MutationSet<'a> {
    /// A set of mutations and their respective probabilities
    mutations: Vec<(Float, Box<dyn Mutation<'a>>)>,

    // The total probability accumulated so far
    total_prob : Float,    
}

impl <'a>MutationSet<'a> {
    
    /// Adds a mutation and its probability. 
    /// 
    /// Note that the probabilities are relative to each other, so they can add up to
    /// more than 1. 
    pub fn push(&mut self, probability: Float, mutation: Box<dyn Mutation<'a>>){        
        self.total_prob += probability;
        // We store the accumulated probability
        self.mutations.push( (self.total_prob, mutation) );
    }


    /// Calculates the probability of accepting a mutation
    pub fn prob_of_accept(&self, fx1: Spectrum, fx2: Spectrum) -> Float {        

        // Nothing can be worse... mutate
        if fx1.is_black(){
            return 1.            
        }
        let fx1 = fx1.radiance();
        let fx2 = fx2.radiance();
        // return
        (fx2 / fx1).min(1.)        
        
    }
}

impl <'a>Mutation<'a> for MutationSet<'a> {
    
    fn mutate(&self, x: &Path,  scene: &'a Scene, camera: &dyn Camera, min_length: usize, rng: &mut SmallRng) -> Path<'a> {
        let p : Float = rng.gen();
        let p = p * self.total_prob;
        for (acc_prob, mutation) in &self.mutations{
            if p < *acc_prob {
                return mutation.mutate(x, scene, camera, min_length, rng)
            }
        }
        unreachable!();

    }
}








pub struct RestartRay {}
impl <'a>Mutation<'a> for RestartRay {    
    fn mutate(&self, _x: &Path,  scene: &'a Scene, camera: &dyn Camera, min_length: usize, rng: &mut SmallRng) -> Path<'a> {
        
        let camera_sample = camera.gen_random_sample(rng);
        let (primary_ray, _weight) = camera.gen_ray(&camera_sample);
        
        
        // Create a random path
        Path::new_from_random_walk(&primary_ray, scene, min_length, rng)
    }
}

pub struct LocalExploration {}
impl <'a>Mutation<'a> for LocalExploration {    
    fn mutate(&self, x: &Path,  scene: &'a Scene, camera: &dyn Camera, min_length: usize, rng: &mut SmallRng) -> Path<'a> {
        todo!()
    }
}
