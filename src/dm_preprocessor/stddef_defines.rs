use std::collections::HashMap;

use super::{define_definition::DmDefineDefinition, DmPreProcessor};

impl DmPreProcessor {
    pub(super) fn initial_defines() -> HashMap<String, DmDefineDefinition> {
        let mut defines = HashMap::new();

        // DM_VERSION and DM_BUILD should match the version of BYOND we are compatible with.
        DmDefineDefinition::new_basic_replace("DM_VERSION", &["515".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DM_BUILD", &["1609".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("NORTH", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SOUTH", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("EAST", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("WEST", &["8".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("NORTHEAST", &["5".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("NORTHWEST", &["9".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SOUTHEAST", &["6".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SOUTHWEST", &["10".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("UP", &["16".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DOWN", &["32".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("SEEINVIS", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEEMOBS", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEEOBJS", &["8".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEETURFS", &["16".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("BLIND", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEE_MOBS", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEE_OBJS", &["8".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEE_TURFS", &["16".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEE_SELF", &["32".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEE_INFRA", &["64".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEE_PIXELS", &["256".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEE_THRU", &["512".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SEE_BLACKNESS", &["1024".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOB_PERSPECTIVE", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("EYE_PERSPECTIVE", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("EDGE_PERSPECTIVE", &["2".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("LEGACY_MOVEMENT_MODE", &["0".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("TILE_MOVEMENT_MODE", &["1".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PIXEL_MOVEMENT_MODE", &["2".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("FLOAT_LAYER", &["-1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("AREA_LAYER", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("TURF_LAYER", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("OBJ_LAYER", &["3".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOB_LAYER", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FLY_LAYER", &["5".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("EFFECTS_LAYER", &["5000".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("TOPDOWN_LAYER", &["10000".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("BACKGROUND_LAYER", &["20000".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FLOAT_PLANE", &["-32767".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("TOPDOWN_MAP", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ISOMETRIC_MAP", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SIDE_MAP", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("TILED_ICON_MAP", &["32768".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("NO_STEPS", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FORWARD_STEPS", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SLIDE_STEPS", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SYNC_STEPS", &["3".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("LONG_GLIDE", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("RESET_COLOR", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("RESET_ALPHA", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("RESET_TRANSFORM", &["8".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("NO_CLIENT_COLOR", &["16".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("KEEP_TOGETHER", &["32".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("KEEP_APART", &["64".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PLANE_MASTER", &["128".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("TILE_BOUND", &["256".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PIXEL_SCALE", &["512".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PASS_MOUSE", &["1024".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("TILE_MOVER", &["2048".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("TRUE", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FALSE", &["0".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("MALE", &["\"male\"".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FEMALE", &["\"female\"".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("NEUTER", &["\"neuter\"".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PLURAL", &["\"plural\"".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("MOUSE_INACTIVE_POINTER", &["0".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_ACTIVE_POINTER", &["1".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_DRAG_POINTER", &["3".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_DROP_POINTER", &["4".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_ARROW_POINTER", &["5".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_CROSSHAIRS_POINTER", &["6".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_HAND_POINTER", &["7".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("MOUSE_LEFT_BUTTON", &["1".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_RIGHT_BUTTON", &["2".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_MIDDLE_BUTTON", &["4".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_CTRL_KEY", &["8".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_SHIFT_KEY", &["16".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MOUSE_ALT_KEY", &["32".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("CONTROL_FREAK_ALL", &["1".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("CONTROL_FREAK_SKIN", &["2".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("CONTROL_FREAK_MACROS", &["4".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("MS_WINDOWS", &["\"MS Windows\"".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("UNIX", &["\"UNIX\"".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("_DM_datum", &["0x001".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_atom", &["0x002".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_movable", &["0x004".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_sound", &["0x020".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_image", &["0x040".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_Icon", &["0x100".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_RscFile", &["0x200".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_Matrix", &["0x400".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_Database", &["0x1000".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_Regex", &["0x2000".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_Special", &["0x4000".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("_DM_Wrapper", &["0x8000".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("SOUND_MUTE", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SOUND_PAUSED", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SOUND_STREAM", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SOUND_UPDATE", &["16".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("ICON_ADD", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ICON_SUBTRACT", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ICON_MULTIPLY", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ICON_OVERLAY", &["3".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ICON_AND", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ICON_OR", &["5".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ICON_UNDERLAY", &["6".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("MATRIX_COPY", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MATRIX_MULTIPLY", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MATRIX_ADD", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MATRIX_SUBTRACT", &["3".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MATRIX_INVERT", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MATRIX_ROTATE", &["5".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MATRIX_SCALE", &["6".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MATRIX_TRANSLATE", &["7".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MATRIX_INTERPOLATE", &["8".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MATRIX_MODIFY", &["128".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("LINEAR_EASING", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SINE_EASING", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("CIRCULAR_EASING", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("CUBIC_EASING", &["3".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("BOUNCE_EASING", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ELASTIC_EASING", &["5".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("BACK_EASING", &["6".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("QUAD_EASING", &["7".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("JUMP_EASING", &["8".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("EASE_IN", &["64".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("EASE_OUT", &["128".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("ANIMATION_END_NOW", &["1".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ANIMATION_LINEAR_TRANSFORM", &["2".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ANIMATION_PARALLEL", &["4".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ANIMATION_SLICE", &["8".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ANIMATION_RELATIVE", &["256".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("ANIMATION_CONTINUE", &["512".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("BLEND_DEFAULT", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("BLEND_OVERLAY", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("BLEND_ADD", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("BLEND_SUBTRACT", &["3".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("BLEND_MULTIPLY", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("BLEND_INSET_OVERLAY", &["5".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("VIS_INHERIT_ICON", &["1".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("VIS_INHERIT_ICON_STATE", &["2".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("VIS_INHERIT_DIR", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("VIS_INHERIT_LAYER", &["8".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("VIS_INHERIT_PLANE", &["16".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("VIS_INHERIT_ID", &["32".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("VIS_UNDERLAY", &["64".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("VIS_HIDE", &["128".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("DATABASE_OPEN", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_CLOSE", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_ERROR_CODE", &["2".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_ERROR", &["3".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_QUERY_CLEAR", &["4".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_QUERY_ADD", &["5".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_QUERY_EXEC", &["8".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_QUERY_NEXT", &["9".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_QUERY_ABORT", &["10".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_QUERY_RESET", &["11".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_QUERY_ROWS_AFFECTED", &["12".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_ROW_COLUMN_NAMES", &["16".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_ROW_COLUMN_VALUE", &["17".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("DATABASE_ROW_LIST", &["18".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("WAVE_SIDEWAYS", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("WAVE_BOUNDED", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MASK_INVERSE", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("MASK_SWAP", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("OUTLINE_SHARP", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("OUTLINE_SQUARE", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FILTER_OVERLAY", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FILTER_UNDERLAY", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FILTER_COLOR_RGB", &["0".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FILTER_COLOR_HSV", &["1".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FILTER_COLOR_HSL", &["2".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("FILTER_COLOR_HCY", &["3".into()])
            .insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("COLORSPACE_RGB", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("COLORSPACE_HSV", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("COLORSPACE_HSL", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("COLORSPACE_HCY", &["3".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("PROFILE_STOP", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PROFILE_CLEAR", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PROFILE_AVERAGE", &["4".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PROFILE_START", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PROFILE_REFRESH", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("PROFILE_RESTART", &["2".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("UNIFORM_RAND", &["0".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("NORMAL_RAND", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("LINEAR_RAND", &["2".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("SQUARE_RAND", &["3".into()]).insert_into_map(&mut defines);

        DmDefineDefinition::new_basic_replace("JSON_PRETTY_PRINT", &["1".into()])
            .insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("JSON_STRICT", &["1".into()]).insert_into_map(&mut defines);
        DmDefineDefinition::new_basic_replace("JSON_ALLOW_COMMENTS", &["2".into()])
            .insert_into_map(&mut defines);

        defines
    }
}
