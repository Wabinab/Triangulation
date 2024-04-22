export enum Routes {
  TNlist = "/template/nlist",
  TNew   = "/template/new",
  TEdit  = "/template/edit",
  TDel   = "/template/delete",
  TClone = "/template/clone",
  TVer   = "/template/version/newest",
  Ts     = "/templates",
  TNameonly  = "/templates/nameonly",

  Pi      = "/pipeline",
  PiProj  = "/pipeline/proj",
  PiNew0  = "/pipeline/0/new",
  PiEdit0 = "/pipeline/0/edit",
  PiDel0  = "/pipeline/0/delete",
  PiNew1  = "/pipeline/1/new",
  PiEdit1 = "/pipeline/1/edit",
  PiDel1  = "/pipeline/0/delete",  // same as 0. 
  PiNew2  = "/pipeline/2/new",
  PiEdit2 = "/pipeline/2/edit",
  PiDel2  = "/pipeline/0/delete",  // same as 0.

  P      = "/project",
  PNew   = "/project/new",
  PEdit  = "/project/edit",
  PEditUnsafe = "/project/edit/unsafe_ver",
  PDel   = "/project/delete",
  PClone = "/project/clone",
  Ps     = "/projects",

  R      = "/response",
  REdit  = "/response/edit",
  RDel   = "/response/delete",
  REditKelly = "/response/edit/kelly",
  RDelKelly = "/response/delete/kelly",

  CNew   = "/cycle/new",
  CEdit  = "/cycle/edit",
  CDel   = "/cycle/delete",
  CClear = "/cycle/clear",

  GenFilename = "/gen_filename"
}
