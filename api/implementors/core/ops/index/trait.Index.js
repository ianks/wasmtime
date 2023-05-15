(function() {var implementors = {
"cranelift_codegen":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;<a class=\"struct\" href=\"cranelift_codegen/ir/entities/struct.Block.html\" title=\"struct cranelift_codegen::ir::entities::Block\">Block</a>&gt; for <a class=\"struct\" href=\"cranelift_codegen/ir/dfg/struct.Blocks.html\" title=\"struct cranelift_codegen::ir::dfg::Blocks\">Blocks</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;<a class=\"struct\" href=\"cranelift_codegen/ir/entities/struct.Inst.html\" title=\"struct cranelift_codegen::ir::entities::Inst\">Inst</a>&gt; for <a class=\"struct\" href=\"cranelift_codegen/ir/dfg/struct.Insts.html\" title=\"struct cranelift_codegen::ir::dfg::Insts\">Insts</a>"]],
"cranelift_entity":[["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;K&gt; for <a class=\"struct\" href=\"cranelift_entity/struct.SecondaryMap.html\" title=\"struct cranelift_entity::SecondaryMap\">SecondaryMap</a>&lt;K, V&gt;<span class=\"where fmt-newline\">where\n    K: <a class=\"trait\" href=\"cranelift_entity/trait.EntityRef.html\" title=\"trait cranelift_entity::EntityRef\">EntityRef</a>,\n    V: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;K&gt; for <a class=\"struct\" href=\"cranelift_entity/struct.BoxedSlice.html\" title=\"struct cranelift_entity::BoxedSlice\">BoxedSlice</a>&lt;K, V&gt;<span class=\"where fmt-newline\">where\n    K: <a class=\"trait\" href=\"cranelift_entity/trait.EntityRef.html\" title=\"trait cranelift_entity::EntityRef\">EntityRef</a>,</span>"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;K&gt; for <a class=\"struct\" href=\"cranelift_entity/struct.PrimaryMap.html\" title=\"struct cranelift_entity::PrimaryMap\">PrimaryMap</a>&lt;K, V&gt;<span class=\"where fmt-newline\">where\n    K: <a class=\"trait\" href=\"cranelift_entity/trait.EntityRef.html\" title=\"trait cranelift_entity::EntityRef\">EntityRef</a>,</span>"]],
"cranelift_isle":[["impl&lt;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;K</a>&gt; for <a class=\"struct\" href=\"cranelift_isle/struct.StableMap.html\" title=\"struct cranelift_isle::StableMap\">StableMap</a>&lt;K, V&gt;"]],
"wasmtime":[["impl&lt;I, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;I&gt; for <a class=\"struct\" href=\"wasmtime/struct.StoreContextMut.html\" title=\"struct wasmtime::StoreContextMut\">StoreContextMut</a>&lt;'_, T&gt;<span class=\"where fmt-newline\">where\n    StoreData: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;I&gt;,</span>"],["impl&lt;I, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;I&gt; for <a class=\"struct\" href=\"wasmtime/struct.StoreContext.html\" title=\"struct wasmtime::StoreContext\">StoreContext</a>&lt;'_, T&gt;<span class=\"where fmt-newline\">where\n    StoreData: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;I&gt;,</span>"]],
"wasmtime_environ":[["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeUnionIndex.html\" title=\"struct wasmtime_environ::component::TypeUnionIndex\">TypeUnionIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/struct.SignatureIndex.html\" title=\"struct wasmtime_environ::SignatureIndex\">SignatureIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/struct.ModuleTypes.html\" title=\"struct wasmtime_environ::ModuleTypes\">ModuleTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeModuleIndex.html\" title=\"struct wasmtime_environ::component::TypeModuleIndex\">TypeModuleIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeVariantIndex.html\" title=\"struct wasmtime_environ::component::TypeVariantIndex\">TypeVariantIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeFuncIndex.html\" title=\"struct wasmtime_environ::component::TypeFuncIndex\">TypeFuncIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeOptionIndex.html\" title=\"struct wasmtime_environ::component::TypeOptionIndex\">TypeOptionIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;T&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"wasmtime_environ/struct.ModuleTypes.html\" title=\"struct wasmtime_environ::ModuleTypes\">ModuleTypes</a>: <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;T&gt;,</span>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeEnumIndex.html\" title=\"struct wasmtime_environ::component::TypeEnumIndex\">TypeEnumIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeListIndex.html\" title=\"struct wasmtime_environ::component::TypeListIndex\">TypeListIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl&lt;K: <a class=\"trait\" href=\"wasmtime_environ/trait.EntityRef.html\" title=\"trait wasmtime_environ::EntityRef\">EntityRef</a>, V&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;K&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/dfg/struct.Intern.html\" title=\"struct wasmtime_environ::component::dfg::Intern\">Intern</a>&lt;K, V&gt;"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeComponentInstanceIndex.html\" title=\"struct wasmtime_environ::component::TypeComponentInstanceIndex\">TypeComponentInstanceIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeComponentIndex.html\" title=\"struct wasmtime_environ::component::TypeComponentIndex\">TypeComponentIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeResultIndex.html\" title=\"struct wasmtime_environ::component::TypeResultIndex\">TypeResultIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeTupleIndex.html\" title=\"struct wasmtime_environ::component::TypeTupleIndex\">TypeTupleIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;T&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypesBuilder.html\" title=\"struct wasmtime_environ::component::ComponentTypesBuilder\">ComponentTypesBuilder</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>: <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;T&gt;,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;T&gt; for <a class=\"struct\" href=\"wasmtime_environ/struct.ModuleTypesBuilder.html\" title=\"struct wasmtime_environ::ModuleTypesBuilder\">ModuleTypesBuilder</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"wasmtime_environ/struct.ModuleTypes.html\" title=\"struct wasmtime_environ::ModuleTypes\">ModuleTypes</a>: <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;T&gt;,</span>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeFlagsIndex.html\" title=\"struct wasmtime_environ::component::TypeFlagsIndex\">TypeFlagsIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"],["impl <a class=\"trait\" href=\"wasmtime_environ/__core/ops/trait.Index.html\" title=\"trait wasmtime_environ::__core::ops::Index\">Index</a>&lt;<a class=\"struct\" href=\"wasmtime_environ/component/struct.TypeRecordIndex.html\" title=\"struct wasmtime_environ::component::TypeRecordIndex\">TypeRecordIndex</a>&gt; for <a class=\"struct\" href=\"wasmtime_environ/component/struct.ComponentTypes.html\" title=\"struct wasmtime_environ::component::ComponentTypes\">ComponentTypes</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()