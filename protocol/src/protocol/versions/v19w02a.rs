protocol_packet_ids!(
    handshake Handshaking {
        serverbound Serverbound {
            0x00 => Handshake
        }
        clientbound Clientbound {
        }
    }
    play Play {
        serverbound Serverbound {
            0x00 => TeleportConfirm
            0x01 => QueryBlockNBT
            0x02 => ChatMessage
            0x03 => ClientStatus
            0x04 => ClientSettings
            0x05 => TabComplete
            0x06 => ConfirmTransactionServerbound
            0x07 => EnchantItem
            0x08 => ClickWindow
            0x09 => CloseWindow
            0x0a => PluginMessageServerbound
            0x0b => EditBook_Item
            0x0c => QueryEntityNBT
            0x0d => UseEntity_Hand
            0x0e => KeepAliveServerbound_i64
            0x0f => Player
            0x10 => PlayerPosition
            0x11 => PlayerPositionLook
            0x12 => PlayerLook
            0x13 => VehicleMove
            0x14 => SteerBoat
            0x15 => PickItem
            0x16 => CraftRecipeRequest
            0x17 => ClientAbilities_f32
            0x18 => PlayerDigging
            0x19 => PlayerAction
            0x1a => SteerVehicle
            0x1b => CraftingBookData
            0x1c => NameItem
            0x1d => ResourcePackStatus
            0x1e => AdvancementTab
            0x1f => SelectTrade
            0x20 => SetBeaconEffect
            0x21 => HeldItemChange
            0x22 => UpdateCommandBlock
            0x23 => UpdateCommandBlockMinecart
            0x24 => CreativeInventoryAction
            0x25 => UpdateStructureBlock
            0x26 => SetSign
            0x27 => ArmSwing
            0x28 => SpectateTeleport
            0x29 => PlayerBlockPlacement_f32
            0x2a => UseItem
        }
        clientbound Clientbound {
            0x00 => SpawnObject
            0x01 => SpawnExperienceOrb
            0x02 => SpawnGlobalEntity
            0x03 => SpawnMob_WithMeta
            0x04 => SpawnPainting_String
            0x05 => SpawnPlayer_f64
            0x06 => Animation
            0x07 => Statistics
            0x08 => BlockBreakAnimation
            0x09 => UpdateBlockEntity_u8
            0x0a => BlockAction
            0x0b => BlockChange_VarInt
            0x0c => BossBar
            0x0d => ServerDifficulty
            0x0e => ServerMessage_Position
            0x0f => MultiBlockChange_VarInt
            0x10 => TabCompleteReply
            0x11 => DeclareCommands
            0x12 => ConfirmTransaction
            0x13 => WindowClose
            0x14 => WindowOpenHorse
            0x15 => WindowItems_i16
            0x16 => WindowProperty
            0x17 => WindowSetSlot
            0x18 => SetCooldown
            0x19 => PluginMessageClientbound
            0x1a => NamedSoundEffect
            0x1b => Disconnect
            0x1c => EntityAction
            0x1d => NBTQueryResponse
            0x1e => Explosion_i32
            0x1f => ChunkUnload
            0x20 => ChangeGameState
            0x21 => KeepAliveClientbound_i64
            0x22 => ChunkData_HeightMap
            0x23 => Effect
            0x24 => Particle_f32
            0x25 => JoinGame_i32
            0x26 => Maps
            0x27 => Entity
            0x28 => EntityMove_i16
            0x29 => EntityLookAndMove_i16
            0x2a => EntityLook_VarInt
            0x2b => VehicleTeleport
            0x2c => OpenBook
            0x2d => SignEditorOpen
            0x2e => CraftRecipeResponse
            0x2f => PlayerAbilities
            0x30 => CombatEvent
            0x31 => PlayerInfo
            0x32 => FacePlayer
            0x33 => TeleportPlayer_WithConfirm
            0x34 => EntityUsedBed
            0x35 => UnlockRecipes_WithSmelting
            0x36 => EntityDestroy
            0x37 => EntityRemoveEffect
            0x38 => ResourcePackSend
            0x39 => Respawn_Gamemode
            0x3a => EntityHeadLook
            0x3b => SelectAdvancementTab
            0x3c => WorldBorder
            0x3d => Camera
            0x3e => SetCurrentHotbarSlot
            0x3f => ScoreboardDisplay
            0x40 => EntityMetadata
            0x41 => EntityAttach
            0x42 => EntityVelocity
            0x43 => EntityEquipment_VarInt
            0x44 => SetExperience
            0x45 => UpdateHealth
            0x46 => ScoreboardObjective
            0x47 => SetPassengers
            0x48 => Teams_u8
            0x49 => UpdateScore
            0x4a => SpawnPosition_NoAngle
            0x4b => TimeUpdate
            0x4d => StopSound
            0x4e => SoundEffect
            0x4f => EntitySoundEffect
            0x50 => PlayerListHeaderFooter
            0x51 => CollectItem
            0x52 => EntityTeleport_f64
            0x53 => Advancements
            0x54 => EntityProperties_VarInt
            0x55 => EntityEffect
            0x56 => DeclareRecipes
            0x57 => Tags_WithEntities
            0x58 => UpdateLight_NoTrust
            0x59 => WindowOpen_VarInt
            0x5a => TradeList_WithoutRestock // TODO: without 1.14 added fields
        }
    }
    login Login {
        serverbound Serverbound {
            0x00 => LoginStart
            0x01 => EncryptionResponse
            0x02 => LoginPluginResponse
        }
        clientbound Clientbound {
            0x00 => LoginDisconnect
            0x01 => EncryptionRequest
            0x02 => LoginSuccess_String
            0x03 => SetInitialCompression
            0x04 => LoginPluginRequest
        }
    }
    status Status {
        serverbound Serverbound {
            0x00 => StatusRequest
            0x01 => StatusPing
        }
        clientbound Clientbound {
            0x00 => StatusResponse
            0x01 => StatusPong
        }
    }
);
