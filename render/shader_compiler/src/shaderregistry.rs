#![allow(unused_variables)]
use makepad_live_parser::LiveRegistry;
use makepad_live_parser::Id;
use makepad_live_parser::IdUnpack;
use makepad_live_parser::LiveError;
use makepad_live_parser::LiveErrorOrigin;
use makepad_live_parser::LiveValue;
use makepad_live_parser::Span;
use makepad_live_parser::CrateModule;
use makepad_live_parser::IdPack;
use makepad_live_parser::id_pack;
use makepad_live_parser::id;
use makepad_live_parser::live_error_origin;
use makepad_live_parser::IdFmt;
use makepad_live_parser::FullNodePtr;
use makepad_live_parser::LocalNodePtr;
use makepad_live_parser::LiveDocument;
use makepad_live_parser::Token;
use crate::shaderast::DrawShaderDecl;
use crate::shaderast::StructDecl;
use crate::shaderast::Decl;
use crate::shaderast::FnDecl;
use crate::shaderast::TextureDecl;
use crate::shaderast::InstanceDecl;
use crate::shaderast::UniformDecl;
use crate::shaderast::TyExpr;
use crate::shaderast::TyExprKind;
use crate::shaderast::TyLit;
use crate::shaderast::Ident;
use crate::shaderast::FnNodePtr;
use crate::shaderast::VarDefNodePtr;
use crate::shaderparser::ShaderParser;
use crate::shaderparser::ShaderParserDep;
use crate::shaderast::InputNodePtr;
use crate::shaderast::StructNodePtr;
use crate::shaderast::ShaderNodePtr;
use crate::shaderast::ValueNodePtr;
use crate::shaderast::ConstNodePtr;
use crate::shaderast::FnSelfKind;
use crate::shaderast::ConstDecl;
use std::fmt;
use std::cell::Cell;
use std::collections::HashMap;
use crate::builtin::Builtin;
use crate::builtin::generate_builtins;
use crate::analyse::StructAnalyser;
use crate::analyse::FnDefAnalyser;
use crate::analyse::ConstAnalyser;
use crate::analyse::ShaderCompileOptions;
use crate::env::Env;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ShaderResourceId(CrateModule, Id);

impl fmt::Display for ShaderResourceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct ShaderRegistry {
    pub live_registry: LiveRegistry,
    pub shader_compile_options: ShaderCompileOptions,
    pub consts: HashMap<ConstNodePtr, ConstDecl>,
    pub plain_fns: HashMap<FnNodePtr, FnDecl>,
    pub draw_shaders: HashMap<ShaderNodePtr, DrawShaderDecl>,
    pub structs: HashMap<StructNodePtr, StructDecl>,
    
    pub draw_inputs: HashMap<ShaderResourceId, ShaderDrawInput>,
    pub builtins: HashMap<Ident, Builtin>,
}

impl ShaderRegistry {
    pub fn new() -> Self {
        Self {
            shader_compile_options: ShaderCompileOptions{
                gather_all: false,
                create_const_table: false,
                no_const_collapse: false
            },
            live_registry: LiveRegistry::default(),
            structs: HashMap::new(),
            consts: HashMap::new(),
            draw_shaders: HashMap::new(),
            plain_fns: HashMap::new(),
            draw_inputs: HashMap::new(),
            builtins: generate_builtins()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ShaderDrawInput {
    pub uniforms: Vec<ShaderDrawInputDef>,
    pub instances: Vec<ShaderDrawInputDef>,
    pub textures: Vec<ShaderDrawInputDef>,
}

#[derive(Clone, Debug)]
pub struct ShaderDrawInputDef {
    pub ident: Ident,
    pub ty_expr: TyExpr
}

impl ShaderDrawInput {
    pub fn add_uniform(&mut self, name: &str, ty_expr: TyExpr) {
        if let TyExprKind::Lit {ty_lit, ..} = ty_expr.kind {
            if ty_lit == TyLit::Texture2D {
                self.textures.push(ShaderDrawInputDef {ident: Ident(Id::from_str(name)), ty_expr});
                return
            }
        }
        self.uniforms.push(ShaderDrawInputDef {ident: Ident(Id::from_str(name)), ty_expr});
    }
    
    pub fn add_instance(&mut self, modpath: &str, cls: &str, name: &str, ty_expr: TyExpr) {
        self.instances.push(ShaderDrawInputDef {ident: Ident(Id::from_str(name)), ty_expr});
    }
}

pub enum LiveNodeFindResult {
    NotFound,
    Component(FullNodePtr),
    Struct(StructNodePtr),
    Function(FnNodePtr),
    PossibleStatic(StructNodePtr, FnNodePtr),
    Const(ConstNodePtr),
    LiveValue(ValueNodePtr, TyLit)
}

impl ShaderRegistry {
    
    pub fn find_live_node_by_path(&self, base_ptr: FullNodePtr, ids: &[Id]) -> LiveNodeFindResult {
        // what are the types of things we can find.
        
        fn no_ids(ids: &[Id], result: LiveNodeFindResult) -> LiveNodeFindResult {
            if ids.len() == 0 {result} else {LiveNodeFindResult::NotFound}
        }
        
        fn var_def_is_const(doc:&LiveDocument, token_start:u32, base_ptr: FullNodePtr)->LiveNodeFindResult{
            if Token::Ident(id!(const)) == doc.tokens[token_start as usize].token{
                LiveNodeFindResult::Const(ConstNodePtr(base_ptr))            
            }
            else{
                LiveNodeFindResult::NotFound      
            }
        }
        
        let (doc, node) = self.live_registry.resolve_ptr(base_ptr);
        match node.value {
            LiveValue::Bool(_) => return no_ids(ids, LiveNodeFindResult::LiveValue(ValueNodePtr(base_ptr), TyLit::Bool)),
            LiveValue::Int(_) => return no_ids(ids, LiveNodeFindResult::LiveValue(ValueNodePtr(base_ptr), TyLit::Int)),
            LiveValue::Float(_) => return no_ids(ids, LiveNodeFindResult::LiveValue(ValueNodePtr(base_ptr), TyLit::Float)),
            LiveValue::Color(_) => return no_ids(ids, LiveNodeFindResult::LiveValue(ValueNodePtr(base_ptr), TyLit::Vec4)),
            LiveValue::Vec2(_) => return no_ids(ids, LiveNodeFindResult::LiveValue(ValueNodePtr(base_ptr), TyLit::Vec2)),
            LiveValue::Vec3(_) => return no_ids(ids, LiveNodeFindResult::LiveValue(ValueNodePtr(base_ptr), TyLit::Vec3)),
            LiveValue::Fn {..} => return no_ids(ids, LiveNodeFindResult::Function(FnNodePtr(base_ptr))),
            LiveValue::VarDef {token_start, ..} =>  return no_ids(ids, var_def_is_const(doc, token_start, base_ptr)),
            LiveValue::Class {class, node_start: ns, node_count: nc, ..} => {
                if ids.len() == 0 { // check if we are struct or component
                    match self.live_registry.find_base_class_id(class) {
                        Some(id_pack!(Struct)) => return LiveNodeFindResult::Struct(StructNodePtr(base_ptr)),
                        _ => return LiveNodeFindResult::Component(base_ptr),
                    }
                }
                // now we bounce around the DOM tree.
                let mut parent_ptr = base_ptr.local_ptr;
                let mut node_start = ns as usize;
                let mut node_count = nc as usize;
                let mut level = base_ptr.local_ptr.level + 1;
                for i in 0..ids.len() {
                    let id = ids[i];
                    let mut found = false;
                    for j in 0..node_count {
                        let node = &doc.nodes[level][j + node_start];
                        if node.id_pack == IdPack::single(id) {
                            // we found the node.
                            let node_ptr = LocalNodePtr {level: level, index: j + node_start};
                            if i == ids.len() - 1 { // last item
                                let full_node_ptr = FullNodePtr {file_id: base_ptr.file_id, local_ptr: node_ptr};
                                match node.value {
                                    LiveValue::Class {class, ..} => {
                                        match self.live_registry.find_base_class_id(class) {
                                            Some(id_pack!(Struct)) => return LiveNodeFindResult::Struct(StructNodePtr(full_node_ptr)),
                                            Some(id_pack!(Component)) => return LiveNodeFindResult::Component(full_node_ptr),
                                            _ => return LiveNodeFindResult::NotFound
                                        }
                                    },
                                    LiveValue::Fn {..} => { // check if its a method or a free roaming function
                                        let full_base_ptr = FullNodePtr {file_id: base_ptr.file_id, local_ptr: parent_ptr};
                                        let base_node = doc.resolve_ptr(parent_ptr);
                                        if let LiveValue::Class {class, ..} = base_node.value {
                                            // lets check if our base is a component or a struct
                                            match self.live_registry.find_base_class_id(class) {
                                                Some(id_pack!(Struct)) => return LiveNodeFindResult::PossibleStatic(
                                                    StructNodePtr(full_base_ptr),
                                                    FnNodePtr(full_node_ptr)
                                                ),
                                                _ => return LiveNodeFindResult::Function(FnNodePtr(full_node_ptr)),
                                            }
                                        }
                                        else {
                                            panic!()
                                        }
                                    },
                                    LiveValue::Bool(_) => return LiveNodeFindResult::LiveValue(ValueNodePtr(full_node_ptr), TyLit::Bool),
                                    LiveValue::Int(_) => return LiveNodeFindResult::LiveValue(ValueNodePtr(full_node_ptr), TyLit::Int),
                                    LiveValue::Float(_) => return LiveNodeFindResult::LiveValue(ValueNodePtr(full_node_ptr), TyLit::Float),
                                    LiveValue::Color(_) => return LiveNodeFindResult::LiveValue(ValueNodePtr(full_node_ptr), TyLit::Vec4),
                                    LiveValue::Vec2(_) => return LiveNodeFindResult::LiveValue(ValueNodePtr(full_node_ptr), TyLit::Vec2),
                                    LiveValue::Vec3(_) => return LiveNodeFindResult::LiveValue(ValueNodePtr(full_node_ptr), TyLit::Vec3),
                                    LiveValue::VarDef {token_start, ..} =>  return var_def_is_const(doc, token_start, full_node_ptr),
                                    _ => return LiveNodeFindResult::NotFound
                                }
                            }
                            else { // we need to be either an object or a class
                                parent_ptr = node_ptr;
                                level += 1;
                                match node.value {
                                    LiveValue::Class {node_start: ns, node_count: nc, ..} => {
                                        node_start = ns as usize;
                                        node_count = nc as usize;
                                    },
                                    _ => return LiveNodeFindResult::NotFound
                                }
                                found = true;
                                break
                            }
                        }
                    }
                    if !found {
                        break
                    }
                }
            }
            _ => ()
        }
        LiveNodeFindResult::NotFound
    }
    
    pub fn register_draw_input(&mut self, mod_path: &str, name: &str, draw_input: ShaderDrawInput) {
        let parts = mod_path.split("::").collect::<Vec<_ >> ();
        if parts.len()>2 {
            panic!("submodules not supported");
        }
        let crate_module = CrateModule(
            Id::from_str(parts[0]),
            Id::from_str(if parts.len()>1 {parts[1]}else {"main"})
        );
        let name_id = Id::from_str(name);
        self.draw_inputs.insert(ShaderResourceId(crate_module, name_id), draw_input);
    }
    
    pub fn parse_shader_resource_id_from_multi_id(crate_id: Id, module_id: Id, span: Span, target: IdPack, multi_ids: &[Id]) -> Result<ShaderResourceId, LiveError> {
        match target.unpack() {
            IdUnpack::Multi {index, count} => {
                if count == 2 {
                    let part1 = multi_ids[index + 0];
                    let part2 = multi_ids[index + 1];
                    if part1 != id!(self) {
                        return Err(LiveError {
                            origin: live_error_origin!(),
                            span: span,
                            message: format!("Unsupported target naming {}", IdFmt::col(multi_ids, target))
                        });
                    }
                    // ok so we have to find crate_id, module_id, part2
                    return Ok(ShaderResourceId(CrateModule(crate_id, module_id), part2))
                }
                if count == 3 {
                    let part1 = multi_ids[index + 0];
                    let part2 = multi_ids[index + 1];
                    let part3 = multi_ids[index + 1];
                    return Ok(ShaderResourceId(CrateModule(if part1 == id!(crate) {crate_id}else {part1}, part2), part3));
                }
            }
            _ => ()
        }
        return Err(LiveError {
            origin: live_error_origin!(),
            span: span,
            message: format!("Unsupported target naming {}", IdFmt::col(multi_ids, target))
        });
    }
    
    pub fn analyse_deps(&mut self, deps: &[ShaderParserDep])-> Result<(), LiveError>{
        // recur on used types
        for dep in deps {
            match dep{
                ShaderParserDep::Const(dep)=>{
                    self.analyse_const(*dep) ?;
                },
                ShaderParserDep::Struct(dep)=>{
                    self.analyse_struct(*dep) ?;
                },
                ShaderParserDep::Function(struct_ptr, fn_ptr)=>{
                    self.analyse_plain_fn(*struct_ptr, *fn_ptr)?
                }
            }
        }
        Ok(())
    }
    
       // lets compile the thing
    pub fn analyse_const(&mut self, const_ptr: ConstNodePtr) -> Result<(), LiveError> {
        if self.consts.get(&const_ptr).is_some() {
            return Ok(());
        }
        let (doc, const_node) = self.live_registry.resolve_ptr(const_ptr.0);
        match const_node.value {
            LiveValue::VarDef {token_start, token_count, scope_start, scope_count} => {
                let mut parser_deps = Vec::new();
                let id = const_node.id_pack.unwrap_single();
                let mut parser = ShaderParser::new(
                    self,
                    doc.get_tokens(token_start, token_count + 1),
                    doc.get_scopes(scope_start, scope_count),
                    &mut parser_deps,
                    None,
                    //Some(struct_full_ptr)
                );
                
                let const_decl = parser.expect_const_decl(Ident(id)) ?;
                self.consts.insert(const_ptr, const_decl);
                
                self.analyse_deps(&parser_deps)?;
                
                let mut env = Env::new(self);
                let mut ca = ConstAnalyser {
                    decl: self.consts.get(&const_ptr).unwrap(),
                    env: &mut env,
                    options: self.shader_compile_options,
                };
                ca.analyse_const_decl() ?;
            }
            _=>panic!()
        }
        return Ok(())
    }
    
     // lets compile the thing
    pub fn analyse_plain_fn(&mut self, struct_ptr: Option<StructNodePtr>, fn_ptr: FnNodePtr) -> Result<(), LiveError> {
        
        if self.plain_fns.get(&fn_ptr).is_some() {
            return Ok(());
        }
        // alright lets parse and analyse a plain fn
        let (doc, fn_node) = self.live_registry.resolve_ptr(fn_ptr.0);
        match fn_node.value {
            LiveValue::Fn {token_start, token_count, scope_start, scope_count} => {
                let id = fn_node.id_pack.unwrap_single();
                let mut parser_deps = Vec::new();
                // lets parse this thing
                let mut parser = ShaderParser::new(
                    self,
                    doc.get_tokens(token_start, token_count + 1),
                    doc.get_scopes(scope_start, scope_count),
                    &mut parser_deps,
                    if let Some(struct_ptr) = struct_ptr{Some(FnSelfKind::Struct(struct_ptr))}else{None},
                    //Some(struct_full_ptr)
                );
                
                let fn_decl = parser.expect_plain_fn_decl(
                    fn_ptr,
                    Ident(id),
                ) ?;
                self.plain_fns.insert(fn_ptr, fn_decl);
                
                self.analyse_deps(&parser_deps)?;
                
                // ok analyse the struct methods now.
                let mut env = Env::new(self);
                let mut fa = FnDefAnalyser {
                    decl: self.plain_fns.get(&fn_ptr).unwrap(),
                    env: &mut env,
                    is_inside_loop: false,
                    options: ShaderCompileOptions {
                        gather_all: false,
                        create_const_table: false,
                        no_const_collapse: false
                    }
                };
                fa.analyse_fn_decl() ?;
                fa.analyse_fn_def() ?;
                
                Ok(())
            }
            _=>panic!()
        }
    }
        
    
    // lets compile the thing
    pub fn analyse_struct(&mut self, struct_ptr: StructNodePtr) -> Result<(), LiveError> {
        
        if self.structs.get(&struct_ptr).is_some() {
            return Ok(());
        }
        
        let (doc, class_node) = self.live_registry.resolve_ptr(struct_ptr.0);
        //let doc = &self.live_registry.expanded[full_ptr.file_id.to_index()];
        //let class_node = &doc.nodes[full_ptr.local_ptr.level][full_ptr.local_ptr.index];
        
        match class_node.value {
            LiveValue::Class {node_start, node_count, class} => {
                let mut struct_decl = StructDecl {
                    span: self.live_registry.token_id_to_span(class_node.token_id),
                    // ident: Ident(class_node.id_pack.unwrap_single()),
                    fields: Vec::new(),
                    methods: Vec::new()
                    //    struct_body: ShaderBody::default()
                };
                
                let mut parser_deps = Vec::new();
                for i in 0..node_count as usize {
                    let prop_ptr = FullNodePtr {file_id: struct_ptr.0.file_id, local_ptr: LocalNodePtr {
                        level: struct_ptr.0.local_ptr.level + 1,
                        index: i + node_start as usize
                    }};
                    let prop = doc.resolve_ptr(prop_ptr.local_ptr);
                    match prop.value {
                        LiveValue::VarDef {token_start, token_count, scope_start, scope_count} => {
                            let id = prop.id_pack.unwrap_single();
                            let mut parser = ShaderParser::new(
                                self,
                                doc.get_tokens(token_start, token_count + 1),
                                doc.get_scopes(scope_start, scope_count),
                                &mut parser_deps,
                                Some(FnSelfKind::Struct(struct_ptr))
                                //Some(struct_full_ptr)
                            );
                            // we only allow a field def
                            let decl = parser.expect_field(Ident(id), VarDefNodePtr(prop_ptr)) ?;
                            struct_decl.fields.push(decl);
                        },
                        LiveValue::Fn {token_start, token_count, scope_start, scope_count} => {
                            let id = prop.id_pack.unwrap_single();
                            // lets parse this thing
                            let mut parser = ShaderParser::new(
                                self,
                                doc.get_tokens(token_start, token_count + 1),
                                doc.get_scopes(scope_start, scope_count),
                                &mut parser_deps,
                                Some(FnSelfKind::Struct(struct_ptr))
                                //Some(struct_full_ptr)
                            );
                            
                            let fn_decl = parser.expect_method_decl(
                                FnNodePtr(prop_ptr),
                                Ident(id),
                            ) ?;
                            // if we get false, this was not a method but could be static.
                            // statics need a pointer to their struct to resolve Self
                            // so we can't treat them purely as loose methods
                            if let Some(fn_decl) = fn_decl{
                                struct_decl.methods.push(fn_decl)
                            }
                        }
                        _ => {
                            return Err(LiveError {
                                origin: live_error_origin!(),
                                span: self.live_registry.token_id_to_span(prop.token_id),
                                message: format!("Cannot use {:?} in struct", prop.value)
                            })
                        }
                    }
                }
                // we should store the structs
                self.structs.insert(struct_ptr, struct_decl);
                
                self.analyse_deps(&parser_deps)?;

                // ok analyse the struct methods now.
                let mut env = Env::new(self);
                let mut sa = StructAnalyser {
                    struct_decl: self.structs.get(&struct_ptr).unwrap(),
                    env: &mut env,
                    options: ShaderCompileOptions {
                        gather_all: false,
                        create_const_table: false,
                        no_const_collapse: false
                    }
                };
                sa.analyse_struct() ?;
                println!("STRUCT");
                
            }
            _ => ()
        }
        
        Ok(())
    }
    
    // lets compile the thing
    pub fn analyse_draw_shader(&mut self, crate_id: Id, module_id: Id, ids: &[Id]) -> Result<DrawShaderDecl, LiveError> {
        // lets find the FullPointer
        
        if let Some(class_full_ptr) = self.live_registry.find_full_node_ptr_from_ids(crate_id, module_id, ids) {
            let mut draw_shader_decl = DrawShaderDecl::default();
            // we have a pointer to the thing to instance.
            let (doc, class_node) = self.live_registry.resolve_ptr(class_full_ptr);
            
            match class_node.value {
                LiveValue::Class {node_start, node_count, ..} => {
                    let mut parser_deps = Vec::new();
                    let mut draw_input_srid = None;
                    for i in 0..node_count as usize {
                        let prop_ptr = FullNodePtr {file_id: class_full_ptr.file_id, local_ptr: LocalNodePtr {
                            level: class_full_ptr.local_ptr.level + 1,
                            index: i + node_start as usize
                        }};
                        let prop = doc.resolve_ptr(prop_ptr.local_ptr);
                        match prop.value {
                            LiveValue::ResourceRef {target} => {
                                // draw input or default_geometry
                                match prop.id_pack {
                                    id_pack!(draw_input) => {
                                        let srid = Self::parse_shader_resource_id_from_multi_id(
                                            crate_id,
                                            module_id,
                                            self.live_registry.token_id_to_span(prop.token_id),
                                            target,
                                            &doc.multi_ids
                                        ) ?;
                                        draw_input_srid = Some((srid, self.live_registry.token_id_to_span(prop.token_id)))
                                    },
                                    id_pack!(default_geometry) => {
                                        // this thing needs to have 3 seg
                                        let srid = Self::parse_shader_resource_id_from_multi_id(
                                            crate_id,
                                            module_id,
                                            self.live_registry.token_id_to_span(prop.token_id),
                                            target,
                                            &doc.multi_ids
                                        ) ?;
                                        draw_shader_decl.default_geometry = Some(srid);
                                    },
                                    _ => { // unknown
                                        return Err(LiveError {
                                            origin: live_error_origin!(),
                                            span: self.live_registry.token_id_to_span(prop.token_id),
                                            message: format!("Unknown var ref type {}", prop.id_pack)
                                        })
                                    }
                                }
                            }
                            LiveValue::VarDef {token_start, token_count, scope_start, scope_count} => {
                                if let IdUnpack::Single(id) = prop.id_pack.unpack() {
                                    let mut parser = ShaderParser::new(
                                        self,
                                        doc.get_tokens(token_start, token_count + 1),
                                        doc.get_scopes(scope_start, scope_count),
                                        &mut parser_deps,
                                        Some(FnSelfKind::Shader(ShaderNodePtr(class_full_ptr)))
                                        //None
                                    );
                                    let decl = parser.expect_other_decl(Ident(id), prop_ptr) ?;
                                    if let Some(decl) = decl{
                                        draw_shader_decl.decls.push(decl);
                                    }
                                    //else{ // it was a const
                                    //}
                                }
                            },
                            LiveValue::Fn {token_start, token_count, scope_start, scope_count} => {
                                if let IdUnpack::Single(id) = prop.id_pack.unpack() {
                                    // lets parse this thing
                                    let mut parser = ShaderParser::new(
                                        self,
                                        doc.get_tokens(token_start, token_count + 1),
                                        doc.get_scopes(scope_start, scope_count),
                                        &mut parser_deps,
                                        Some(FnSelfKind::Shader(ShaderNodePtr(class_full_ptr)))
                                        //None
                                    );
                                    
                                    let fn_decl = parser.expect_method_decl(
                                        FnNodePtr(prop_ptr),
                                        Ident(id),
                                    ) ?;
                                    if let Some(fn_decl) = fn_decl{
                                        draw_shader_decl.methods.push(fn_decl)
                                    }
                                }
                            }
                            _ => ()
                        }
                    }
                    // if we have a draw_input process it.
                    if let Some((draw_input_srid, span)) = draw_input_srid {
                        if let Some(draw_input) = self.draw_inputs.get(&draw_input_srid) {
                            for decl in &draw_shader_decl.decls {
                                if let Decl::Instance(_) = decl {
                                    return Err(LiveError {
                                        origin: live_error_origin!(),
                                        span,
                                        message: format!("Cannot use both instance defs and draw_input {}", draw_input_srid)
                                    })
                                }
                            }
                            for instance in &draw_input.instances {
                                draw_shader_decl.decls.push(
                                    Decl::Instance(InstanceDecl {
                                        is_used_in_fragment_shader: Cell::new(None),
                                        input_node_ptr: InputNodePtr::Class(class_full_ptr),
                                        span,
                                        ident: instance.ident,
                                        ty_expr: instance.ty_expr.clone(),
                                    })
                                )
                            }
                            
                            for uniform in &draw_input.uniforms {
                                draw_shader_decl.decls.push(
                                    Decl::Uniform(UniformDecl {
                                        block_ident: Ident(id!(default)),
                                        input_node_ptr: InputNodePtr::Class(class_full_ptr),
                                        span,
                                        ident: uniform.ident,
                                        ty_expr: uniform.ty_expr.clone(),
                                    })
                                )
                            }
                            
                            for texture in &draw_input.textures {
                                draw_shader_decl.decls.push(
                                    Decl::Texture(TextureDecl {
                                        span,
                                        input_node_ptr: InputNodePtr::Class(class_full_ptr),
                                        ident: texture.ident,
                                        ty_expr: texture.ty_expr.clone(),
                                    })
                                )
                            }
                        }
                        else {
                            return Err(LiveError {
                                origin: live_error_origin!(),
                                span,
                                message: format!("Cannot find draw_input {}", draw_input_srid)
                            })
                        }
                        
                        
                    }
                    
                    self.analyse_deps(&parser_deps)?;
                    
                    // ok we have all structs
                    return Ok(draw_shader_decl)
                }
                _ => ()
            }
        }
        return Err(LiveError {
            origin: live_error_origin!(),
            span: Span::default(),
            message: format!("analyse_draw_shader could not find {} {} {} ", crate_id, module_id, ids[0])
        })
    }
    
}
