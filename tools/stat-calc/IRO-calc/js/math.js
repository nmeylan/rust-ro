function Min(x, y)
{
	if (x > y)
		return y;
	return x;
}

function Max(x, y)
{
	if (x < y)
		return y;
	return x;
}

function Between(x, y, z)
{
	if (z < x) // wrong parameters
		return 0;
	if (y < x) // lower limit
		return x;
	if (y > z) // higher limit
		return z;
	return y; // between
}

function sort(work)
{ // Bubble Sort ... (recursion doesn't work with js)
	var num=0;
	if(Language != 0)
		num = Language * 2 + 1;

	for(var i=1;work[i]!="EOF";i++)
	{
		for(var k=i;k>0;k--)
		{
			if(ITEM_NAME[work[k-1]][num] > ITEM_NAME[work[k]][num])
			{
				var work_backup = work[k-1];
				work[k-1] = work[k];
				work[k] = work_backup;
			}
		}
	}
	return work;
}

n_NtoS =["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
n_NtoS2 =["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","0","1","2","3","4","5","6","7","8","9"];

function NtoS(n,keta)
{ // [n] to char, keta = digits
	var strX = "";
	if(keta == 3)
	{
		strX += n_NtoS[Math.floor(n / 100)];
		var w = n % 100;
		if(w >= 10)
			strX += w;
		else
			strX += "0" + w;
	}
	else if(keta == 2)
	{
		strX += n_NtoS[Math.floor(n / 10)];
		strX += n % 10;
	}
	else
	{
		strX += n_NtoS[n];
	}
	return strX;
}

function NtoS01(wb,wc,wd,we,wf)
{ // binary mask from inputs
	var n = 0;
	if(wb == true)
		n += 16;
	if(wc == true)
		n += 8;
	if(wd == true)
		n += 4;
	if(we == true)
		n += 2;
	if(wf == true)
		n += 1;
	return NtoS2(n,1);
}

function NtoS2(n,keta)
{ // Number (digits) to string ?
	var strX = "";
	if(keta == 3)
	{
		strX += n_NtoS2[Math.floor(n / 3844)];
		strX += n_NtoS2[Math.floor(n % 3844 / 62)];
		strX += n_NtoS2[n % 62];
	}
	else if(keta == 2)
	{
		strX += n_NtoS2[Math.floor(n / 62)];
		strX += n_NtoS2[n % 62];
	}
	else
	{
		strX += n_NtoS2[n];
	}
	return strX;
}

function NtoS05(wa,wb)
{ // 6*wa + wb
	var n;
	n = wa * 6;
	n += wb;
	return NtoS2(n,1);
}

function StoN(n)
{ // String to Number
	n += "";
	for(var i=0;i<=61;i++)
		if(n == n_NtoS[i])
			return i;
}

function StoNx(n)
{ // String to Numbers [Array2]
	n += "";
	for(var i=0;i<=61;i++)
		if(n == n_NtoS2[i])
			return i;
}

function StoN2(n)
{ // String to Numbers [62 ?]
	n += "";
	var keta = n.length;
	if (keta == 3)
	{
		var w = n.charAt(0);
		var x = StoNx(w) * 62 * 62;
		w = n.charAt(1);
		x += StoNx(w) * 62;
		w = n.charAt(2);
		x += StoNx(w);
	}
	else if(keta == 2)
	{
		var w = n.charAt(0);
		var x = StoNx(w) * 62;
		w = n.charAt(1);
		x += StoNx(w);
	}
	else
	{
		var w = n.charAt(0);
		var x = StoNx(w);
	}
	return x;
}

function NumSearch(NS1,NS2)
{ // return (NS1 in Array NS2)
	var end = NS2.length-1;
	for(var i=0;i<=end;i++)
	{
		if(NS1 == NS2[i])
			return 1;
	}
	return 0;
}
