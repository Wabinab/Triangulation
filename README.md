# Triangulation
If you ever need to triangulate, to put words into actions, to implement what books and articles and blogs told you, mesh and connect multiple ideas into one aggregate, this is the ultimate app to connect them together! 

## Download
Download the latest in the [Release tab](https://github.com/Wabinab/Triangulation/releases) in Github. Currently, only windows is tested, while linux is imagined (runs on windows subsystem on linux, though), mac... one don't use mac sorry. The linux probably run on mac. 

---
## How to Use The Program
Start off by creating a **New Template**. For each template, you can create stage at the sidebar. **After you create a stage**, the button to create a reminder, an investment, or a checklist card will be available. How you want to do it, it's up to you. Just like you can drink water by holding a cup up to your mouth, you could also drink water by holding the cup, twist your arm 360° around your neck, before bringing the cup to your mouth. There are no restrictions how you're gonna execute the program. 

Once you're done with creating a template, you can go back to the home page, and create a **New Project.** You should select the template which your project should use. In this case, probably the template you created earlier. 

> Even though author built this program, author oneself is figuring out how to use it. 

### How to Update Project To Use Another Template Version
If you already created a project with an existing template, and you change the template in the meantime, the version of the template will be updated. If you now create a new project, it'll use the latest version of the template. However, if you decide to update an existing project to use the template, you could change the version by clicking on the pencil on the version. 

> **_Note! Changing the version might cause the program to break. You MUST ENSURE that you don't break it, one don't provide guarantees._** In the case you cannot save it normally, it means data loss will occur, and if you don't mind data loss, you can use the red "Unsafe Save" button beside the blue "Safe" button. Both will ask for confirmation before saving, so don't worry if you accidentally clicked it. 

### How do I change the name of the `current` stage? 
Double clicking on the name will work. Furthermore, clicking `F2` allow you to edit the name of the _currently selected_ stage. Pressing `ESC` **will save** the new name. 

**Advanced user note**: F2 only works if you didn't disable the button in your windows registry or equivalent. 

### How about changing the name of cycles? 
Similar to how you edit stage, double click or F2, then ESC to save. 

### What's the difference between `clone` (double square) and `export` in template? 
Clone will clone the same template, adding "(Clone)" to the back of the name, to another template in your template folder. You can then modify the name and whatever yourself. Useful if you already have a scaffolding, but wanted to minutely tweak the difference from there instead of creating from scratch. Similarly for projects. 

Export is designed for layman user who probably don't know how to find where the folder with the template is located, plus even if they found it, it's not in a name that they can easily understand. Therefore, the export function allow to copy the file to where they want to save it, in their own name. 

**Advanced user note:** Your data is located in the data folder where you run the app. `template` is template, `project` is project. The rest, don't touch. If anything is changed manually from the rest of the folder, the program might break. All template and project are displayed on screen in reverse order, so you have to count the number and what page you're on, do some maths, before you can find it. 

### I'm in the "New Project" and "New Template" page, but cannot go back home clicking the Triangulation symbol on top right!
It's not a bug. The template is part of "home", so, of course, you can't go back "home". Instead, press the "cancel" button to return home. Or if you want, reload the page (pressing F5) would also work. 

---

## Features of the Program
**Reminders** as the main card feature, plans for one to answer one's own questions, through one's mind of thoughts. It integrates most features of a Google Form, but less sophisticated, depending on your needs. The extra "No Answer" are for introductions. 

There is a limit to how many questions one can ask per reminder, after which one should create a new reminder to split them apart. 

As for moving the questions, the "drag and drop" was too difficult to implement, at least one couldn't ensure its stability if it was implemented with existing scaffoldings. Instead, a simple numbering method allows you to move its position to where you want it to, with a little mastering and imagination in the user's brain, of course. 

**Investment** or **Kelly** was first created to calculate Kelly Criterion and Brier Score, since one just read _Superforecasting_ then, and _Antifragile_ where Nassim Taleb mentioned of Kelly's. For Kelly to be visible, though, you need to have at least one win and at least one loss; if you're a super-investor and always had higher return on investment; than being a speculator; it won't show. Brier score are for predicting the market in the future, **_to discourage you from making market predictions_** because anything equal and more than 0.5 would mean you do better than the flip of a coin; and anything around 0.35-0.5 you cannot be sure you're beating the market or you're lucky; while anything lower, you're just **patient** enough to wait for the return of investment to come. And patient is the key. 

**Checklist** are just plain old checklist. The advantage of checklist is it allows you to define compulsory materials when you first create them in the _template_; but add extra stuffs later in the _project_. 

To prevent the cards from closing accidentally, one prevented clicking outside to close the popup window (what we developers called "modal"). Instead, you either have to save, or have to cancel (the "x" button). If you cancel, it would ask whether to save whatever you change (or in reminder's case, even if you didn't change anything, you'll be prompted due to its complexity); both yes and no will close the window, one save before close, another just close. Or, you can press the "ESC" key to close it, no problem, which would discard all changes you made. 

For reminders and checklist, you can create new "cycle," which is basically an empty sheet to answer the same question more than once. There is a limit to how many "cycles" you can create, after which you would need to clone the whole project into a new one, or delete old cycle(s). For Checklist, note that anything in the "extra items" would **not** be copied over when you create new cycles. You had to manually add them again and again, again and again; if that's the case, it's probably time to modify the corresponding template and update the version of your project to fit. 

For the projects, you could "clear answers" and "clear cycles," which essentially delete everything and return you a clean slate. You probably want to be very careful with these. It will ask for your confirmation, do think before you proceed. Anything cleared will **not** be retrievable, unless you have a backup somewhere. 

You would notice a "data" folder where you execute the program. Your data was stored inside, as `.json.zl` file, essentially compiled first with `jsonm` before compressing into `zlib` format. This was deliberately designed, for the initial program came with a SQLite database, and one had considered, _is the installation worth it?_ and concluded, _no, it's not worth it._ SQLite brought the ability for better search options, and allow for a larger database of questions, at the expense of complicating the codebase. The target had been, all along, as little dependency as possible -- if possible, no installation unless required for hosting (`http-server` from `npm` is the only one for displaying the frontend). One also found exporting data not as easy as it sounds, even with relationships! Here, one could just copy the file to someone and they could use it. The only problem is one can't create too many projects nor templates, as it used the file system; and too many files would overload the system. That wouldn't be a problem, as one intend to keep it small and personal than shared by a team at the core of its design. Easy to maintain and robust is of utmost importance. 

One would also share templates one create in the future, which you could discover and download in "Sample Template" filter on the home page. This, though, requires you to be _online_, as the information are directly fetched from [Github from this repository](https://github.com/Wabinab/Triangulation_Sample) (the `index.json` file it is). As one update it, you can immediately see it without having to update your own Triangulation software. As of writing, it had 2 useless .json.zl file which one uploaded for testing purposes only; which will be deleted as this program officially goes public. 

The program had 2 languages, English and French. One may incorporate a Chinese translation in the future, we'll see how it goes. 

As usual, one could [report bugs](https://github.com/Wabinab/Triangulation/issues) in the Github issues, and one'll check it when one had the time to. Though, do note that if it's a feature request, one might want to earn some money if it's specifically targeted for your purpose, which you'd [contact one on upwork](https://www.upwork.com/freelancers/~011a1c60f09fff5f90?mp_source=share) or [on fiverr](https://www.fiverr.com/s/rVzq57); otherwise, if it's easy and you can convince me to implement it within an hour or so, one could probably share it free for all. Do report bugs though, one would like to know what mistakes one hadn't encountered in the process of development and using it oneself. 