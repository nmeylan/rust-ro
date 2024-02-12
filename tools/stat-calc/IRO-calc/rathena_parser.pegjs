/*
 * Ragnarok Grammar
 * ==========================
 *
 * Example: { bonus bStr,2; bonus bInt,4; bonus bDex,1; bonus2 bAddEle,Ele_Neutral,10; bonus3 bAutoSpell,"MG_COLDBOLT",3,50; bonus4 bAutoSpellOnSkill,"MG_COLDBOLT","MG_FIREBOLT",3,1000; bonus4 bAutoSpellOnSkill,"MG_FIREBOLT","MG_LIGHTNINGBOLT",3,1000; bonus4 bAutoSpellOnSkill,"MG_LIGHTNINGBOLT","WZ_EARTHSPIKE",3,1000; }
 * Example2: { bonus bAllStats,1; bonus bAtk,readparam(bStr)/20; bonus bMatk,readparam(bInt)/20; bonus2 bSubEle,Ele_Neutral,readparam(bVit)/20; bonus bLongAtkRate,readparam(bDex)/20; bonus bCritAtkRate,readparam(bLuk)/20; }
 * Example3: { skill "BS_GREED",1; .@r = getrefine(); if(.@r>6) { if(readparam(bStr)>=90){ bonus bBaseAtk,20; } if(readparam(bInt)>=90){ bonus bMatk,30; } if(readparam(bVit)>=90){ bonus2 bSubEle,Ele_Neutral,10; } if(readparam(bAgi)>=90){ bonus bAspdRate,8; } if(readparam(bDex)>=90){ bonus bLongAtkRate,5; } if(readparam(bLuk)>=90){ bonus bCritAtkRate,10; } } if(.@r>8) { if(readparam(bStr)>=90){ bonus bBaseAtk,10; } if(readparam(bInt)>=90){ bonus bMatk,20; } if(readparam(bVit)>=90){ bonus2 bSubEle,Ele_Neutral,5; } if(readparam(bAgi)>=90){ bonus bAspd,1; } if(readparam(bDex)>=90){ bonus bLongAtkRate,5; } if(readparam(bLuk)>=90){ bonus bCritAtkRate,5; } } }
 */

Expression
  = [{] _ expr:Expression2 _ [}] {
    var result = {};
    for (var i = 0; i < expr.length; i++) {
      var obj = expr[i];
      for (var key in obj) {
          if (obj.hasOwnProperty(key)) {
              if (result[key]) {
                  // Temporary
                result[key] = obj[key];
                } else {
                result[key] = obj[key];
                }
            }
        }
    }
  return result;
}

Expression2
  = head:StatList tail:(_ (Expression2))* {
  var current = tail, result = head, i;

  if (tail[0] === undefined) {
    return [head];
  } else {
    return [head].concat(tail[0][1]);
  }
}

StatList
  = bonus:( ConditionalBranchs / BonusList) {
    return bonus;
}

BonusList
  = bonus:((VarSet / Skill / Bonus1 / Bonus2 / Bonus3 / Bonus4 / Bonus5) ';') {
    return bonus[0];
}

ConditionalBranchs
  = conds:ConditionalBranchList {
  return { conditions: conds }
}

ConditionalBranchList
  = head:ConditionalBranch tail:(_ ConditionalBranchList)* {
  if (tail[0]) {
    return [head].concat(tail[0][1]);
  } else {
    return [head];
  }
}

ConditionalBranch
  = 'if(' condition:Condition ')' _ '{' _ body:Expression2 _ '}' {
  return { condition: condition, body: body };
}

Condition
  = left:(Variable / Function / Constant) center:Operator right:Integer {
  return [left, center, right].join('');
}

Operator
  = [><=][=]? {
  return text();
}

Function
  = [a-z]+ '(' BonusKeyword ')' {
  return text();
}

VarSet
  = bonus:(Variable _ Equals _ InnerExpression) {
    return { command: bonus.join('') }
}

Variable
  = prefix:".@" varname:[a-z]+ {
  return  "_locals." + varname;
}

Skill
  = bonus:("skill" _ String comma Integer) {
    var l1 = {};
    l1[bonus[2]] = bonus[4];
    return { 'skill' : l1 };
  }

Bonus1
  = bonus:("bonus" _ BonusKeyword comma InnerExpression) {
    var output = {};
    output[bonus[2]] = bonus[4];
    return output;
  }

Bonus2
  = bonus:("bonus2" _ BonusKeyword comma (Keyword/String) comma InnerExpression) {
    var l1 = {}, l2 = {};
    l2[bonus[4]] = bonus[6];
    l1[bonus[2]] = l2;
    return l1;
  }


Bonus3
  = bonus:("bonus3" _ BonusKeyword comma String comma Integer comma Integer) {
    var l1 = {}, l2 = {}, l3 = {};
    l3[bonus[6]] = bonus[8];
    l2[bonus[4]] = bonus[6];
    l1[bonus[2]] = l2;

    return l1;
  }

Bonus4
  = bonus:("bonus4" _ BonusKeyword comma String comma String comma Integer comma Integer) {
    var output = {};
    output[bonus[2]] = [bonus[4], bonus[6], bonus[8]];
    return output;
  }

Bonus5
  = bonus:("bonus5" _ BonusKeyword comma String comma Integer comma Integer comma Constant comma Integer) {
    var output = {};
    output[bonus[2]] = [bonus[4], bonus[6], bonus[8]];
    return output;
  }

String
  = [\"]name:([A-Z_]+)[\"] { return name.join(''); }

Constant
 = [A-Z_]+ { return text(); }

Keyword
  = [A-Za-z_]+ { return text(); }

BonusKeyword
  = "b"[A-Z][A-Za-z]+[0-9]? { return text(); }

InnerExpression
  = expr:(EvaluatedExpression) { return expr; }

EvaluatedExpression
  = [a-zA-Z0-9\/().@+*-]+ { return text(); }

Integer "integer"
  = [0-9]+ { return parseInt(text(), 10); }

_ "whitespace"
  = [ \t\n\r]*

comma
  = [,]

Equals
  = '='
