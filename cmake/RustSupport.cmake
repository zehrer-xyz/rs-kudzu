function(kudzu_add_rust_staticlib TARGET_NAME PACKAGE_NAME LIB_BASENAME)
    if(NOT CARGO_PROGRAM)
        message(FATAL_ERROR "Cargo is required to build Rust migration crates.")
    endif()

    if(CMAKE_BUILD_TYPE STREQUAL "Debug")
        set(KUDZU_RUST_CARGO_PROFILE_DIR "debug")
        set(KUDZU_RUST_CARGO_PROFILE_ARGS "")
    else()
        set(KUDZU_RUST_CARGO_PROFILE_DIR "release")
        set(KUDZU_RUST_CARGO_PROFILE_ARGS "--release")
    endif()

    file(GLOB_RECURSE KUDZU_RUST_SOURCES CONFIGURE_DEPENDS
        "${PROJECT_SOURCE_DIR}/rust/*.rs"
        "${PROJECT_SOURCE_DIR}/rust/*.toml"
        "${PROJECT_SOURCE_DIR}/Cargo.toml")

    set(KUDZU_RUST_LIB_PATH
        "${KUDZU_RUST_TARGET_DIR}/${KUDZU_RUST_CARGO_PROFILE_DIR}/lib${LIB_BASENAME}.a")

    add_custom_command(
        OUTPUT "${KUDZU_RUST_LIB_PATH}"
        COMMAND
            ${CMAKE_COMMAND} -E env
            CARGO_TARGET_DIR=${KUDZU_RUST_TARGET_DIR}
            ${CARGO_PROGRAM} build
            --manifest-path ${PROJECT_SOURCE_DIR}/Cargo.toml
            -p ${PACKAGE_NAME}
            --lib
            ${KUDZU_RUST_CARGO_PROFILE_ARGS}
        WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}
        DEPENDS ${KUDZU_RUST_SOURCES}
        VERBATIM
        COMMAND_EXPAND_LISTS
    )

    add_custom_target(${TARGET_NAME}_build DEPENDS "${KUDZU_RUST_LIB_PATH}")

    add_library(${TARGET_NAME} STATIC IMPORTED GLOBAL)
    set_target_properties(${TARGET_NAME} PROPERTIES IMPORTED_LOCATION "${KUDZU_RUST_LIB_PATH}")
    add_dependencies(${TARGET_NAME} ${TARGET_NAME}_build)
endfunction()

