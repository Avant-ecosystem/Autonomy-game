import { useFrame } from '@react-three/fiber';
import React, {useRef} from 'react'
import { AnimationMixer, LoopRepeat} from 'three';

const CardRoom = ({scene , animations}) => {
    const avatarAnimation =animations[0]
    const mixerRef= useRef()
    if(!mixerRef.current){
        mixerRef.current = new AnimationMixer(scene)

    }
    const mixer = mixerRef.current

    const action = mixer.clipAction(avatarAnimation)
    action.setLoop(LoopRepeat,Infinity)
    action.play()
    useFrame((state,delta)=>{
        mixer.update(delta)
    })
  return <primitive object={scene}   position={[0, -2, 0]} rotation={[0, Math.PI / -3, 0]} animation={animations}/>
}

export default CardRoom