function crack(login) {
  var pw = '';
  while(true) {
    var cands = [...'0123456789'];
    while(cands.length>1) {
      var timing = cands.map(s=>[s,0]);
      for(var r=0; r<3; r++) {
        for(var i=0; i<cands.length; i++) {
          var w = pw+cands[i];
          var t = process.hrtime();
          if(login(w)) return w;
          var s = process.hrtime(t);
          timing[i][1]+=s[1];
        }
      }
      cands = timing.sort((a,b)=>b[1]-a[1]).slice(0,timing.length/2|0).map(e=>e[0]);
    }
    pw += cands[0];
    if(pw.length>32) pw='';
  }
}
